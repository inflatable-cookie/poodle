# Internal Writing Style

Status: active
Audience: agents and operators

## Rule

Use clear, natural prose for internal work and normal thread replies. Keep it
concise and high-signal, but keep the connective language that makes reasoning
easy to follow.

Meaning:

- short
- blunt
- high signal
- enough connective prose to explain a recommendation or trade-off
- no ceremonial phrasing
- no long prose if bullets or fragments work
- no big protocol recap unless it adds real value

## End-of-turn reply shape

Use this structure for the end-of-turn closeout or meaningful checkpoint
reply, not for every message:

1. what changed
2. current state
3. validation, only if it failed or materially affects confidence
4. next move

## Apply to

- chat replies about internal work
- task briefs and PR descriptions
- the plan and triage notes
- internal contracts and working rules

## Do not apply blindly to

- public-facing product copy
- onboarding or tutorial docs
- user-facing explanations where fuller prose helps

## Guardrails

- trim filler, not the connective language that carries the reasoning
- no caveman theatre or status-report theatre; stay clear
- use file refs only where they help
- mention validation only when it failed or materially changes confidence,
  risk, or next action
- do not force the full end-of-turn frame into every message when a shorter
  answer is enough

## Next Task

Point `AGENTS.md` and `CLAUDE.md` here with a short note instead of repeating
the full style rule inline.
