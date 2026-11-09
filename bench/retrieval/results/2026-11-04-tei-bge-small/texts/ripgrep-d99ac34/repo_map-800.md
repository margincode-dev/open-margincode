crates/core/flags/defs.rs:
  L3455: struct Index;
  L3458: fn is_switch(&self) -> bool {
  L3461: fn name_short(&self) -> Option<u8> {
  L3464: fn name_long(&self) -> &'static str {
  L3467: fn doc_category(&self) -> Category {
  L3470: fn doc_short(&self) -> &'static str {
  L3473: fn doc_long(&self) -> &'static str {
  L3511: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L3525: fn test_index() {
  L3554: struct IndexCrud;
  L3557: fn is_switch(&self) -> bool {
  L3560: fn name_long(&self) -> &'static str {
  L3563: fn doc_category(&self) -> Category {
  L3566: fn doc_short(&self) -> &'static str {
  L3569: fn doc_long(&self) -> &'static str {
  L3619: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L3629: fn test_index_crud() {
  L3651: struct IndexForce;
  L3654: fn is_switch(&self) -> bool {
  L3657: fn name_long(&self) -> &'static str {
  L3660: fn doc_category(&self) -> Category {
  L3663: fn doc_short(&self) -> &'static str {
  L3666: fn doc_long(&self) -> &'static str {
  L3673: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L3683: fn test_index_force() {
  L3698: struct IndexPath;
  L3701: fn is_switch(&self) -> bool {
  L3704: fn name_long(&self) -> &'static str {
  L3707: fn doc_variable(&self) -> Option<&'static str> {
  L3710: fn doc_category(&self) -> Category {
  L3713: fn doc_short(&self) -> &'static str {
  L3716: fn doc_long(&self) -> &'static str {
  L3722: fn completion_type(&self) -> CompletionType {
  L3726: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L3735: fn test_index_path() {
  L44: pub(super) const FLAGS: &[&dyn Flag] = &[
  L167: pub(super) fn indexing_unsupported_flag(
  L233: struct AfterContext;
  L236: fn is_switch(&self) -> bool {
  L239: fn name_short(&self) -> Option<u8> {
  L242: fn name_long(&self) -> &'static str {
  L245: fn doc_variable(&self) -> Option<&'static str> {
  L248: fn doc_category(&self) -> Category {
  L251: fn doc_short(&self) -> &'static str {
  L254: fn doc_long(&self) -> &'static str {
  L263: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L271: fn test_after_context() {
  L319: struct AutoHybridRegex;
  L322: fn is_switch(&self) -> bool {
  L325: fn name_long(&self) -> &'static str {
  L328: fn name_negated(&self) -> Option<&'static str> {
