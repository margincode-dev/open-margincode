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
