crates/core/flags/defs.rs:
  L4864: struct NoIgnoreFiles;
  L4867: fn is_switch(&self) -> bool {
  L4870: fn name_long(&self) -> &'static str {
  L4873: fn name_negated(&self) -> Option<&'static str> {
  L4876: fn doc_category(&self) -> Category {
  L4879: fn doc_short(&self) -> &'static str {
  L4882: fn doc_long(&self) -> &'static str {
  L4889: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L4897: fn test_no_ignore_files() {
  L319: struct AutoHybridRegex;
  L322: fn is_switch(&self) -> bool {
  L325: fn name_long(&self) -> &'static str {
  L328: fn name_negated(&self) -> Option<&'static str> {
  L331: fn doc_category(&self) -> Category {
  L334: fn doc_short(&self) -> &'static str {
  L337: fn doc_long(&self) -> &'static str {
  L367: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L380: fn test_auto_hybrid_regex() {
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
  L5454: struct OneFileSystem;
  L5457: fn is_switch(&self) -> bool {
  L5460: fn name_long(&self) -> &'static str {
  L5463: fn name_negated(&self) -> Option<&'static str> {
  L5466: fn doc_category(&self) -> Category {
  L5469: fn doc_short(&self) -> &'static str {
  L5472: fn doc_long(&self) -> &'static str {
  L5492: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L5500: fn test_one_file_system() {
  L5514: struct OnlyMatching;
  L5517: fn is_switch(&self) -> bool {
  L5520: fn name_short(&self) -> Option<u8> {
  L5523: fn name_long(&self) -> &'static str {
  L5526: fn doc_category(&self) -> Category {
  L5529: fn doc_short(&self) -> &'static str {
  L5532: fn doc_long(&self) -> &'static str {
  L5539: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L5548: fn test_only_matching() {
  L6139: struct RegexSizeLimit;
  L6142: fn is_switch(&self) -> bool {
  L6145: fn name_long(&self) -> &'static str {
  L6148: fn doc_variable(&self) -> Option<&'static str> {
  L6151: fn doc_category(&self) -> Category {
  L6154: fn doc_short(&self) -> &'static str {
  L6157: fn doc_long(&self) -> &'static str {
  L6173: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L6182: fn test_regex_size_limit() {
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
  L774: fn name_long(&self) -> &'static str {
  L777: fn doc_variable(&self) -> Option<&'static str> {
  L780: fn doc_category(&self) -> Category {
  L783: fn doc_short(&self) -> &'static str {
  L786: fn doc_long(&self) -> &'static str {
  L837: fn doc_choices(&self) -> &'static [&'static str] {
  L841: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L855: fn test_color() {
  L891: struct Colors;
  L894: fn is_switch(&self) -> bool {
  L897: fn name_long(&self) -> &'static str {
  L900: fn doc_variable(&self) -> Option<&'static str> {
  L903: fn doc_category(&self) -> Category {
  L906: fn doc_short(&self) -> &'static str {
  L909: fn doc_long(&self) -> &'static str {
  L969: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L979: fn test_colors() {
  L1022: struct Column;
  L1025: fn is_switch(&self) -> bool {
  L1028: fn name_long(&self) -> &'static str {
  L1031: fn name_negated(&self) -> Option<&'static str> {
  L1034: fn doc_category(&self) -> Category {
  L1037: fn doc_short(&self) -> &'static str {
  L1040: fn doc_long(&self) -> &'static str {
  L1051: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L1059: fn test_column() {
  L1075: struct Context;
  L1078: fn is_switch(&self) -> bool {
  L1081: fn name_short(&self) -> Option<u8> {
  L1084: fn name_long(&self) -> &'static str {
  L1087: fn doc_variable(&self) -> Option<&'static str> {
  L1090: fn doc_category(&self) -> Category {
  L1093: fn doc_short(&self) -> &'static str {
  L1096: fn doc_long(&self) -> &'static str {
  L1108: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L1116: fn test_context() {
  L1215: struct ContextSeparator;
  L1218: fn is_switch(&self) -> bool {
  L1221: fn name_long(&self) -> &'static str {
  L1224: fn name_negated(&self) -> Option<&'static str> {
  L1227: fn doc_variable(&self) -> Option<&'static str> {
  L1230: fn doc_category(&self) -> Category {
  L1233: fn doc_short(&self) -> &'static str {
  L1236: fn doc_long(&self) -> &'static str {
  L1249: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L1265: fn test_context_separator() {
  L1324: struct Count;
  L1327: fn is_switch(&self) -> bool {
  L1330: fn name_short(&self) -> Option<u8> {
  L1333: fn name_long(&self) -> &'static str {
  L1336: fn doc_category(&self) -> Category {
  L1339: fn doc_short(&self) -> &'static str {
  L1342: fn doc_long(&self) -> &'static str {
  L1371: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L1380: fn test_count() {
  L1399: struct CountMatches;
  L1402: fn is_switch(&self) -> bool {
  L1405: fn name_long(&self) -> &'static str {
  L1408: fn doc_variable(&self) -> Option<&'static str> {
  L1411: fn doc_category(&self) -> Category {
  L1414: fn doc_short(&self) -> &'static str {
  L1417: fn doc_long(&self) -> &'static str {
  L1434: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L1443: fn test_count_matches() {
  L1459: struct Crlf;
  L1462: fn is_switch(&self) -> bool {
  L1465: fn name_long(&self) -> &'static str {
  L1468: fn name_negated(&self) -> Option<&'static str> {
  L1471: fn doc_category(&self) -> Category {
  L1474: fn doc_short(&self) -> &'static str {
  L1477: fn doc_long(&self) -> &'static str {
  L1495: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L1506: fn test_crlf() {
  L1533: struct Debug;
  L1536: fn is_switch(&self) -> bool {
  L1539: fn name_long(&self) -> &'static str {
  L1542: fn doc_category(&self) -> Category {
  L1545: fn doc_short(&self) -> &'static str {
  L1548: fn doc_long(&self) -> &'static str {
  L1561: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L1570: fn test_debug() {
  L1583: struct DfaSizeLimit;
  L1586: fn is_switch(&self) -> bool {
  L1589: fn name_long(&self) -> &'static str {
  L1592: fn doc_variable(&self) -> Option<&'static str> {
  L1595: fn doc_category(&self) -> Category {
  L1598: fn doc_short(&self) -> &'static str {
  L1601: fn doc_long(&self) -> &'static str {
  L1614: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L1623: fn test_dfa_size_limit() {
  L1659: struct Encoding;
  L1662: fn is_switch(&self) -> bool {
  L1665: fn name_short(&self) -> Option<u8> {
  L1668: fn name_long(&self) -> &'static str {
  L1671: fn name_negated(&self) -> Option<&'static str> {
  L1674: fn doc_variable(&self) -> Option<&'static str> {
  L1677: fn doc_category(&self) -> Category {
  L1680: fn doc_short(&self) -> &'static str {
  L1683: fn doc_long(&self) -> &'static str {
  L1703: fn completion_type(&self) -> CompletionType {
  L1707: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L1730: fn test_encoding() {
  L1768: struct Engine;
  L1771: fn is_switch(&self) -> bool {
  L1774: fn name_long(&self) -> &'static str {
  L1777: fn doc_variable(&self) -> Option<&'static str> {
  L1780: fn doc_category(&self) -> Category {
  L1783: fn doc_short(&self) -> &'static str {
  L1786: fn doc_long(&self) -> &'static str {
  L1808: fn doc_choices(&self) -> &'static [&'static str] {
  L1812: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L1827: fn test_engine() {
  L1860: struct FieldContextSeparator;
  L1863: fn is_switch(&self) -> bool {
  L1866: fn name_long(&self) -> &'static str {
  L1869: fn doc_variable(&self) -> Option<&'static str> {
  L1872: fn doc_category(&self) -> Category {
  L1875: fn doc_short(&self) -> &'static str {
  L1878: fn doc_long(&self) -> &'static str {
  L1889: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L1899: fn test_field_context_separator() {
  L1968: struct FieldMatchSeparator;
  L1971: fn is_switch(&self) -> bool {
  L1974: fn name_long(&self) -> &'static str {
  L1977: fn doc_variable(&self) -> Option<&'static str> {
  L1980: fn doc_category(&self) -> Category {
  L1983: fn doc_short(&self) -> &'static str {
  L1986: fn doc_long(&self) -> &'static str {
  L1997: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L2007: fn test_field_match_separator() {
  L2064: struct File;
  L2067: fn is_switch(&self) -> bool {
  L2070: fn name_short(&self) -> Option<u8> {
  L2073: fn name_long(&self) -> &'static str {
  L2076: fn doc_variable(&self) -> Option<&'static str> {
  L2079: fn doc_category(&self) -> Category {
  L2082: fn doc_short(&self) -> &'static str {
  L2085: fn doc_long(&self) -> &'static str {
  L2101: fn completion_type(&self) -> CompletionType {
  L2105: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L2114: fn test_file() {
  L2362: struct FixedStrings;
  L2365: fn is_switch(&self) -> bool {
  L2368: fn name_short(&self) -> Option<u8> {
  L2371: fn name_long(&self) -> &'static str {
  L2374: fn name_negated(&self) -> Option<&'static str> {
  L2377: fn doc_category(&self) -> Category {
  L2380: fn doc_short(&self) -> &'static str {
  L2383: fn doc_long(&self) -> &'static str {
  L2391: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L2399: fn test_fixed_strings() {
  L2418: struct Follow;
  L2421: fn is_switch(&self) -> bool {
  L2424: fn name_short(&self) -> Option<u8> {
  L2427: fn name_long(&self) -> &'static str {
  L2430: fn name_negated(&self) -> Option<&'static str> {
  L2433: fn doc_category(&self) -> Category {
  L2436: fn doc_short(&self) -> &'static str {
  L2439: fn doc_long(&self) -> &'static str {
  L2449: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L2457: fn test_follow() {
  L2476: struct Generate;
  L2479: fn is_switch(&self) -> bool {
  L2482: fn name_long(&self) -> &'static str {
  L2485: fn doc_variable(&self) -> Option<&'static str> {
  L2488: fn doc_category(&self) -> Category {
  L2491: fn doc_short(&self) -> &'static str {
  L2494: fn doc_long(&self) -> &'static str {
  L2519: fn doc_choices(&self) -> &'static [&'static str] {
  L2529: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L2545: fn test_generate() {
  L2581: struct Glob;
  L2584: fn is_switch(&self) -> bool {
  L2587: fn name_short(&self) -> Option<u8> {
  L2590: fn name_long(&self) -> &'static str {
  L2593: fn doc_variable(&self) -> Option<&'static str> {
  L2596: fn doc_category(&self) -> Category {
  L2599: fn doc_short(&self) -> &'static str {
  L2602: fn doc_long(&self) -> &'static str {
  L2631: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L2640: fn test_glob() {
  L2671: struct GlobCaseInsensitive;
  L2674: fn is_switch(&self) -> bool {
  L2677: fn name_long(&self) -> &'static str {
  L2680: fn name_negated(&self) -> Option<&'static str> {
  L2683: fn doc_category(&self) -> Category {
  L2686: fn doc_short(&self) -> &'static str {
  L2689: fn doc_long(&self) -> &'static str {
  L2696: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L2704: fn test_glob_case_insensitive() {
  L2728: struct Heading;
  L2731: fn is_switch(&self) -> bool {
  L2734: fn name_long(&self) -> &'static str {
  L2737: fn name_negated(&self) -> Option<&'static str> {
  L2740: fn doc_category(&self) -> Category {
  L2743: fn doc_short(&self) -> &'static str {
  L2746: fn doc_long(&self) -> &'static str {
  L2760: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L2768: fn test_heading() {
  L2787: struct Help;
  L2790: fn is_switch(&self) -> bool {
  L2793: fn name_long(&self) -> &'static str {
  L2796: fn name_short(&self) -> Option<u8> {
  L2799: fn doc_category(&self) -> Category {
  L2802: fn doc_short(&self) -> &'static str {
  L2805: fn doc_long(&self) -> &'static str {
  L2817: fn update(&self, v: FlagValue, _: &mut LowArgs) -> anyhow::Result<()> {
  L2828: fn test_help() {
  L2847: struct Hidden;
  L2850: fn is_switch(&self) -> bool {
  L2853: fn name_short(&self) -> Option<u8> {
  L2856: fn name_long(&self) -> &'static str {
  L2859: fn name_negated(&self) -> Option<&'static str> {
  L2862: fn doc_category(&self) -> Category {
  L2865: fn doc_short(&self) -> &'static str {
  L2868: fn doc_long(&self) -> &'static str {
  L2887: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L2895: fn test_hidden() {
  L2914: struct HostnameBin;
  L2917: fn is_switch(&self) -> bool {
  L2920: fn name_long(&self) -> &'static str {
  L2923: fn doc_variable(&self) -> Option<&'static str> {
  L2926: fn doc_category(&self) -> Category {
  L2929: fn doc_short(&self) -> &'static str {
  L2932: fn doc_long(&self) -> &'static str {
