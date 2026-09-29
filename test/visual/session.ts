import type { BrowserContext, Dialog, FileChooser, Page } from "playwright";

import { pinPage } from "./capture";
import type { Framework } from "./config";
import { ensureUp } from "./server";

/**
 * Shared headless-browser session for preview harnesses.
 *
 * Every harness that drives the Svelte/React previews needs the same
 * infrastructure recovery, and each one used to re-learn it from a timeout:
 *
 *  - a single page degrades after ~15-20 SPA navigations as the vite client
 *    accumulates state, until waits stop settling, so it is recycled on a
 *    fixed cadence;
 *  - a preview started earlier in a long batch can die mid-run, so a dead
 *    preview is restarted before the specimen is blamed;
 *  - a native file chooser or dialog left unhandled can wedge the page, so the
 *    session absorbs both when the page is created;
 *  - one stuck interaction must not consume the whole run, so every attempt
 *    runs under a hard deadline and a timeout poisons the page instead of
 *    awaiting it forever.
 *
 * Previews themselves are always booted through `startPreviews()` in
 * `./server`: it binds the fixed ports with `--strictPort` and fails fast on a
 * squatter instead of trusting the "ready" banner. A harness that reaches for
 * `bun run --cwd packages/<app>/preview dev` directly takes whatever port is
 * free, and a stale server can shadow it.
 *
 * Recovery stays with the caller. Only the caller knows which failures are
 * infrastructure (dead preview, degraded page) and which are evidence (a
 * specimen that genuinely will not settle), and evidence failures must never be
 * retried away.
 */

/** Navigations a single page survives before it must be recycled. */
export const RECYCLE_AFTER = 20;

/** Wall-clock ceiling for one attempt before its page is treated as poisoned. */
export const PAGE_DEADLINE_MS = 120_000;

export class PageDeadlineError extends Error {
  constructor(label: string, deadlineMs: number) {
    super(`${label} exceeded the ${deadlineMs}ms page deadline`);
    this.name = "PageDeadlineError";
  }
}

export type CaptureSessionOptions = {
  context: BrowserContext;
  /** Overrides for tests; every production harness keeps the shared cadence. */
  recycleAfter?: number;
  deadlineMs?: number;
  /** Replaces the default absorb-and-dismiss chooser handling. */
  onFileChooser?: (chooser: FileChooser) => void | Promise<void>;
  /** Replaces the default absorb-and-dismiss dialog handling. */
  onDialog?: (dialog: Dialog) => void | Promise<void>;
};

export class CaptureSession {
  readonly #context: BrowserContext;
  readonly #recycleAfter: number;
  readonly #deadlineMs: number;
  readonly #onFileChooser: (chooser: FileChooser) => void | Promise<void>;
  readonly #onDialog: (dialog: Dialog) => void | Promise<void>;

  #page: Page | null = null;
  #usesOnPage = 0;
  #fileChoosers = 0;
  #dialogs: string[] = [];

  constructor(options: CaptureSessionOptions) {
    this.#context = options.context;
    this.#recycleAfter = options.recycleAfter ?? RECYCLE_AFTER;
    this.#deadlineMs = options.deadlineMs ?? PAGE_DEADLINE_MS;
    this.#onFileChooser =
      options.onFileChooser ??
      ((chooser) => chooser.setFiles([]).catch(() => {}));
    this.#onDialog = options.onDialog ?? ((dialog) => dialog.dismiss().catch(() => {}));
  }

  /** Native file choosers absorbed since the session opened (evidence). */
  get fileChoosers(): number {
    return this.#fileChoosers;
  }

  /** Dialog types absorbed since the session opened (evidence). */
  get dialogs(): readonly string[] {
    return this.#dialogs;
  }

  get page(): Page {
    if (!this.#page) throw new Error("capture session has no page yet; open one with `run`");
    return this.#page;
  }

  /** Close the current page and replace it with a young, pinned one. */
  async recycle(): Promise<void> {
    const previous = this.#page;
    this.#page = null;
    // A deadline-poisoned page can also resist `close`; never await it forever.
    if (previous && !previous.isClosed()) {
      await Promise.race([previous.close().catch(() => {}), sleep(5_000)]);
    }
    const page = await this.#context.newPage();
    page.on("filechooser", (chooser) => {
      this.#fileChoosers += 1;
      void this.#onFileChooser(chooser);
    });
    page.on("dialog", (dialog) => {
      this.#dialogs.push(dialog.type());
      void this.#onDialog(dialog);
    });
    await pinPage(page);
    this.#page = page;
    this.#usesOnPage = 0;
  }

  /**
   * Restart the preview if it stopped answering, then recycle the page.
   * Returns true when the preview had to be restarted.
   */
  async recover(framework: Framework): Promise<boolean> {
    const restarted = await ensureUp(framework);
    await this.recycle();
    return restarted;
  }

  /**
   * Run one attempt against the live page under the per-attempt deadline.
   *
   * A timeout cannot cancel the Playwright call, so the page is poisoned: it is
   * recycled and the attempt fails with `PageDeadlineError`. The caller decides
   * whether to recover and retry.
   */
  async run<T>(label: string, work: (page: Page) => Promise<T>): Promise<T> {
    if (!this.#page || this.#usesOnPage >= this.#recycleAfter) await this.recycle();
    const page = this.page;
    this.#usesOnPage += 1;

    let timer: ReturnType<typeof setTimeout> | undefined;
    // `then` rather than a direct call so a synchronous throw is still a
    // rejection the race handles.
    const attempt = Promise.resolve().then(() => work(page));
    // The loser of the race may settle long after the deadline; keep the
    // rejection handled so it never surfaces as an unhandled promise.
    attempt.catch(() => {});
    const expired = new Promise<never>((_resolve, reject) => {
      timer = setTimeout(
        () => reject(new PageDeadlineError(label, this.#deadlineMs)),
        this.#deadlineMs,
      );
    });

    try {
      return await Promise.race([attempt, expired]);
    } catch (error) {
      if (error instanceof PageDeadlineError) await this.recycle();
      throw error;
    } finally {
      clearTimeout(timer);
    }
  }

  async close(): Promise<void> {
    const page = this.#page;
    this.#page = null;
    if (page && !page.isClosed()) await page.close().catch(() => {});
  }
}

/** Create a session; one per page a harness drives. */
export function captureSession(options: CaptureSessionOptions): CaptureSession {
  return new CaptureSession(options);
}

function sleep(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}
