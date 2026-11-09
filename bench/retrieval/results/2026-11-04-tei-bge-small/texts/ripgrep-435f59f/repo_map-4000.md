crates/core/flags/defs.rs:
  L4313: struct MaxDepth;
  L4316: fn is_switch(&self) -> bool {
  L4319: fn name_short(&self) -> Option<u8> {
  L4322: fn name_long(&self) -> &'static str {
  L4325: fn aliases(&self) -> &'static [&'static str] {
  L4328: fn doc_variable(&self) -> Option<&'static str> {
  L4331: fn doc_category(&self) -> Category {
  L4334: fn doc_short(&self) -> &'static str {
  L4337: fn doc_long(&self) -> &'static str {
  L4351: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L4359: fn test_max_depth() {
  L4864: struct NoIgnoreFiles;
  L4867: fn is_switch(&self) -> bool {
  L4870: fn name_long(&self) -> &'static str {
  L4873: fn name_negated(&self) -> Option<&'static str> {
  L4876: fn doc_category(&self) -> Category {
  L4879: fn doc_short(&self) -> &'static str {
  L4882: fn doc_long(&self) -> &'static str {
  L4889: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L4897: fn test_no_ignore_files() {
  L2208: struct Files;
  L2211: fn is_switch(&self) -> bool {
  L2214: fn name_long(&self) -> &'static str {
  L2217: fn doc_category(&self) -> Category {
  L2220: fn doc_short(&self) -> &'static str {
  L2223: fn doc_long(&self) -> &'static str {
  L2232: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L2241: fn test_files() {
  L2251: struct FilesWithMatches;
  L2254: fn is_switch(&self) -> bool {
  L2257: fn name_short(&self) -> Option<u8> {
  L2260: fn name_long(&self) -> &'static str {
  L2263: fn doc_category(&self) -> Category {
  L2266: fn doc_short(&self) -> &'static str {
  L2269: fn doc_long(&self) -> &'static str {
  L2285: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L2294: fn test_files_with_matches() {
  L2307: struct FilesWithoutMatch;
  L2310: fn is_switch(&self) -> bool {
  L2313: fn name_long(&self) -> &'static str {
  L2316: fn doc_category(&self) -> Category {
  L2319: fn doc_short(&self) -> &'static str {
  L2322: fn doc_long(&self) -> &'static str {
  L2330: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L2342: fn test_files_without_match() {
  L3225: struct IgnoreCase;
  L3228: fn is_switch(&self) -> bool {
  L3231: fn name_short(&self) -> Option<u8> {
  L3234: fn name_long(&self) -> &'static str {
  L3237: fn doc_category(&self) -> Category {
  L3240: fn doc_short(&self) -> &'static str {
  L3243: fn doc_long(&self) -> &'static str {
  L3258: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L3267: fn test_ignore_case() {
  L3286: struct IgnoreFile;
  L3289: fn is_switch(&self) -> bool {
  L3292: fn name_long(&self) -> &'static str {
  L3295: fn doc_variable(&self) -> Option<&'static str> {
  L3298: fn doc_category(&self) -> Category {
  L3301: fn doc_short(&self) -> &'static str {
  L3304: fn doc_long(&self) -> &'static str {
  L3319: fn completion_type(&self) -> CompletionType {
  L3323: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L3332: fn test_ignore_file() {
  L3349: struct IgnoreFileCaseInsensitive;
  L3352: fn is_switch(&self) -> bool {
  L3355: fn name_long(&self) -> &'static str {
  L3358: fn name_negated(&self) -> Option<&'static str> {
  L3361: fn doc_category(&self) -> Category {
  L3364: fn doc_short(&self) -> &'static str {
  L3367: fn doc_long(&self) -> &'static str {
  L3375: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L3383: fn test_ignore_file_case_insensitive() {
  L4136: struct MaxColumns;
  L4139: fn is_switch(&self) -> bool {
  L4142: fn name_short(&self) -> Option<u8> {
  L4145: fn name_long(&self) -> &'static str {
  L4148: fn doc_variable(&self) -> Option<&'static str> {
  L4151: fn doc_category(&self) -> Category {
  L4154: fn doc_short(&self) -> &'static str {
  L4157: fn doc_long(&self) -> &'static str {
  L4166: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L4175: fn test_max_columns() {
  L4194: struct MaxColumnsPreview;
  L4197: fn is_switch(&self) -> bool {
  L4200: fn name_long(&self) -> &'static str {
  L4203: fn name_negated(&self) -> Option<&'static str> {
  L4206: fn doc_category(&self) -> Category {
  L4209: fn doc_short(&self) -> &'static str {
  L4212: fn doc_long(&self) -> &'static str {
  L4226: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L4234: fn test_max_columns_preview() {
  L4249: struct MaxCount;
  L4252: fn is_switch(&self) -> bool {
  L4255: fn name_short(&self) -> Option<u8> {
  L4258: fn name_long(&self) -> &'static str {
  L4261: fn doc_variable(&self) -> Option<&'static str> {
  L4264: fn doc_category(&self) -> Category {
  L4267: fn doc_short(&self) -> &'static str {
  L4270: fn doc_long(&self) -> &'static str {
  L4287: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L4295: fn test_max_count() {
  L4381: struct MaxFilesize;
  L4384: fn is_switch(&self) -> bool {
  L4387: fn name_long(&self) -> &'static str {
  L4390: fn doc_variable(&self) -> Option<&'static str> {
  L4393: fn doc_category(&self) -> Category {
  L4396: fn doc_short(&self) -> &'static str {
  L4399: fn doc_long(&self) -> &'static str {
  L4411: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L4420: fn test_max_filesize() {
  L4696: struct NoIgnore;
  L4699: fn is_switch(&self) -> bool {
  L4702: fn name_long(&self) -> &'static str {
  L4705: fn name_negated(&self) -> Option<&'static str> {
  L4708: fn doc_category(&self) -> Category {
  L4711: fn doc_short(&self) -> &'static str {
  L4714: fn doc_long(&self) -> &'static str {
  L4730: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L4743: fn test_no_ignore() {
  L4768: struct NoIgnoreDot;
  L4771: fn is_switch(&self) -> bool {
  L4774: fn name_long(&self) -> &'static str {
  L4777: fn name_negated(&self) -> Option<&'static str> {
  L4780: fn doc_category(&self) -> Category {
  L4783: fn doc_short(&self) -> &'static str {
  L4786: fn doc_long(&self) -> &'static str {
  L4796: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L4804: fn test_no_ignore_dot() {
  L4817: struct NoIgnoreExclude;
  L4820: fn is_switch(&self) -> bool {
  L4823: fn name_long(&self) -> &'static str {
  L4826: fn name_negated(&self) -> Option<&'static str> {
  L4829: fn doc_category(&self) -> Category {
  L4832: fn doc_short(&self) -> &'static str {
  L4835: fn doc_long(&self) -> &'static str {
  L4842: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L4850: fn test_no_ignore_exclude() {
  L4910: struct NoIgnoreGlobal;
  L4913: fn is_switch(&self) -> bool {
  L4916: fn name_long(&self) -> &'static str {
  L4919: fn name_negated(&self) -> Option<&'static str> {
  L4922: fn doc_category(&self) -> Category {
  L4925: fn doc_short(&self) -> &'static str {
  L4928: fn doc_long(&self) -> &'static str {
  L4936: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L4944: fn test_no_ignore_global() {
  L4958: struct NoIgnoreMessages;
  L4961: fn is_switch(&self) -> bool {
  L4964: fn name_long(&self) -> &'static str {
  L4967: fn name_negated(&self) -> Option<&'static str> {
  L4970: fn doc_category(&self) -> Category {
  L4973: fn doc_short(&self) -> &'static str {
  L4976: fn doc_long(&self) -> &'static str {
  L4985: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L4993: fn test_no_ignore_messages() {
  L5007: struct NoIgnoreParent;
  L5010: fn is_switch(&self) -> bool {
  L5013: fn name_long(&self) -> &'static str {
  L5016: fn name_negated(&self) -> Option<&'static str> {
  L5019: fn doc_category(&self) -> Category {
  L5022: fn doc_short(&self) -> &'static str {
  L5025: fn doc_long(&self) -> &'static str {
  L5034: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L5042: fn test_no_ignore_parent() {
  L5056: struct NoIgnoreVcs;
  L5059: fn is_switch(&self) -> bool {
  L5062: fn name_long(&self) -> &'static str {
  L5065: fn name_negated(&self) -> Option<&'static str> {
  L5068: fn doc_category(&self) -> Category {
  L5071: fn doc_short(&self) -> &'static str {
  L5074: fn doc_long(&self) -> &'static str {
  L5091: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L5099: fn test_no_ignore_vcs() {
  L6588: struct SortFiles;
  L6591: fn is_switch(&self) -> bool {
  L6594: fn name_long(&self) -> &'static str {
  L6597: fn name_negated(&self) -> Option<&'static str> {
  L6600: fn doc_category(&self) -> Category {
  L6603: fn doc_short(&self) -> &'static str {
  L6606: fn doc_long(&self) -> &'static str {
  L6618: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L6630: fn test_sort_files() {
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
  L331: fn doc_category(&self) -> Category {
  L334: fn doc_short(&self) -> &'static str {
  L337: fn doc_long(&self) -> &'static str {
  L367: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L380: fn test_auto_hybrid_regex() {
  L418: struct BeforeContext;
  L421: fn is_switch(&self) -> bool {
  L424: fn name_short(&self) -> Option<u8> {
  L427: fn name_long(&self) -> &'static str {
  L430: fn doc_variable(&self) -> Option<&'static str> {
  L433: fn doc_category(&self) -> Category {
  L436: fn doc_short(&self) -> &'static str {
  L439: fn doc_long(&self) -> &'static str {
  L448: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L456: fn test_before_context() {
  L504: struct Binary;
  L507: fn is_switch(&self) -> bool {
  L510: fn name_long(&self) -> &'static str {
  L513: fn name_negated(&self) -> Option<&'static str> {
  L516: fn doc_category(&self) -> Category {
  L519: fn doc_short(&self) -> &'static str {
  L522: fn doc_long(&self) -> &'static str {
  L558: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L570: fn test_binary() {
  L595: struct BlockBuffered;
  L598: fn is_switch(&self) -> bool {
  L601: fn name_long(&self) -> &'static str {
  L604: fn name_negated(&self) -> Option<&'static str> {
  L607: fn doc_category(&self) -> Category {
  L610: fn doc_short(&self) -> &'static str {
  L613: fn doc_long(&self) -> &'static str {
  L626: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L638: fn test_block_buffered() {
  L655: struct ByteOffset;
  L658: fn is_switch(&self) -> bool {
  L661: fn name_short(&self) -> Option<u8> {
  L664: fn name_long(&self) -> &'static str {
  L667: fn name_negated(&self) -> Option<&'static str> {
  L670: fn doc_category(&self) -> Category {
  L673: fn doc_short(&self) -> &'static str {
  L676: fn doc_long(&self) -> &'static str {
  L688: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L696: fn test_byte_offset() {
  L715: struct CaseSensitive;
  L718: fn is_switch(&self) -> bool {
  L721: fn name_short(&self) -> Option<u8> {
  L724: fn name_long(&self) -> &'static str {
  L727: fn doc_category(&self) -> Category {
  L730: fn doc_short(&self) -> &'static str {
  L733: fn doc_long(&self) -> &'static str {
  L746: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L755: fn test_case_sensitive() {
  L768: struct Color;
  L771: fn is_switch(&self) -> bool {
