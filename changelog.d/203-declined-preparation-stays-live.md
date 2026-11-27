- A drag whose host declines or fails to prepare a cross-window transfer or file
  export now carries on as the same local session. With the pointer still down,
  the source no longer gets a `preparation-declined` or `preparation-failed`
  `onDragEnd` before the local drag, and the session id the host saw is kept.
  Touch hold survives the refusal, and a `prepare` that throws no longer ends
  the gesture (on touch it raised an error). GPUI now continues the drag locally
  too instead of cancelling it. `dragSessionTransition` accepts an optional
  `transport: "local"` on `PREPARE_DECLINED` and `PREPARE_FAILED`; without it
  both still cancel. In `poodle-headless`, the two `DragSessionEvent` variants
  carry the same choice as a `transport` field.
