# Plan 16 M2 stage A agent: Claude Code on the model in $M2_MODEL, in the dream's directory.
exec claude -p "Read DREAM.md and do what it says." \
  --model "$M2_MODEL" --output-format json --max-budget-usd "$M2_RUN_CAP" \
  --permission-mode acceptEdits --allowedTools "Read,Write,Bash" \
  > "claude-result-${AV_DREAM_ROUND:-0}.json"
