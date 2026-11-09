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
