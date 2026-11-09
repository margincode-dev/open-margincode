src/dir_entry.rs:
  L36: pub fn broken_symlink(path: PathBuf, depth: usize) -> Self {
  L44: pub fn path(&self) -> &Path {
  L51: pub fn into_path(self) -> PathBuf {
  L61: pub fn stripped_path(&self, config: &Config) -> &Path {
  L76: pub fn into_stripped_path(self, config: &Config) -> PathBuf {
  L84: pub fn file_type(&self) -> Option<FileType> {
  L100: pub fn depth(&self) -> Option<usize> {
  L142: fn path(&self) -> PathBuf {
  L146: fn file_name(&self) -> OsString {
  L162: fn file_type(&self) -> Option<FileType> {
  L12: enum DirEntryInner {
  L20: pub struct DirEntry {
  L28: pub fn normal(e: ignore::DirEntry) -> Self {
  L91: pub fn metadata(&self) -> Option<&Metadata> {
  L107: pub fn style(&self, ls_colors: &LsColors) -> Option<&Style> {
  L114: fn starts_with_dash(path: &Path) -> bool {
  L120: fn eq(&self, other: &Self) -> bool {
  L129: fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
  L136: fn cmp(&self, other: &Self) -> std::cmp::Ordering {
  L166: fn metadata(&self) -> Option<Metadata> {
  L177: fn dash_prefixed_paths_detected() {
  L184: fn safe_paths_not_flagged() {
src/cli.rs:
  L751: pub fn min_depth(&self) -> Option<usize> {
  L696: pub fn search_paths(&self) -> anyhow::Result<Vec<PathBuf>> {
  L723: fn normalize_path(&self, path: &Path) -> PathBuf {
  L738: pub fn no_search_paths(&self) -> bool {
  L743: pub fn rg_alias_ignore(&self) -> bool {
  L747: pub fn max_depth(&self) -> Option<usize> {
  L803: pub enum FileType {
  L32: pub struct Opts {
  L755: pub fn threads(&self) -> NonZeroUsize {
  L759: pub fn max_results(&self) -> Option<usize> {
  L765: pub fn strip_cwd_prefix<P: FnOnce() -> bool>(&self, auto_pred: P) -> bool {
  L776: pub fn gen_completions(&self) -> anyhow::Result<Option<Shell>> {
  L789: fn default_num_threads() -> NonZeroUsize {
  L826: pub enum ColorWhen {
  L836: pub enum StripCwdWhen {
  L846: pub enum HyperlinkWhen {
  L857: pub struct Exec {
  L862: fn from_arg_matches(matches: &ArgMatches) -> clap::error::Result<Self> {
  L876: fn update_from_arg_matches(&mut self, matches: &ArgMatches) -> clap::error::Result<()> {
  L883: fn augment_args(cmd: Command) -> Command {
  L954: fn augment_args_for_update(cmd: Command) -> Command {
  L959: fn parse_millis(arg: &str) -> Result<Duration, std::num::ParseIntError> {
  L963: fn ensure_current_directory_exists(current_directory: &Path) -> anyhow::Result<()> {
tests/tests.rs:
  L1254: fn test_min_depth_broken_symlink_absolute_path() {
  L1216: fn test_min_depth_broken_symlink() {
  L488: fn test_single_dash_root_path() {
  L1294: fn test_max_depth_broken_symlink() {
  L1333: fn test_exact_depth_broken_symlink() {
  L1787: fn test_symlink_and_full_path_abs_path() {
  L32: fn get_absolute_root_path(env: &TestEnv) -> String {
  L244: fn test_and_plus_ignore_case() {
  L447: fn test_explicit_root_path() {
  L1112: fn test_follow_broken_symlink() {
  L1192: fn test_min_depth() {
  L1683: fn test_symlink_as_root() {
  L1724: fn test_symlink_and_absolute_path() {
  L1745: fn test_symlink_as_absolute_root() {
  L1762: fn test_symlink_and_full_path() {
  L3011: fn test_ignore_contain_precedence_over_depth_check() {
  L3029: fn test_ignore_contain_precedence_over_root_check() {
  L49: fn get_test_env_with_abs_path(dirs: &[&'static str], files: &[&'static str]) -> (TestEnv, String) {
  L56: fn create_file_with_size<P: AsRef<Path>>(path: P, size_in_bytes: usize) {
  L255: fn test_and_plus_case_sensitive() {
  L265: fn test_and_plus_full_path() {
  L316: fn test_multi_file() {
  L340: fn test_multi_file_with_missing() {
  L410: fn test_pattern_with_forward_slash_allowed_with_full_path() {
  L517: fn test_smart_case() {
  L541: fn test_case_sensitive() {
  L556: fn test_case_insensitive() {
  L604: fn test_full_path_glob_searches() {
  L626: fn test_smart_case_glob_searches() {
  L640: fn test_case_sensitive_glob_searches() {
  L667: fn test_full_path() {
  L700: fn test_hidden_file_attribute() {
  L721: fn test_no_ignore() {
  L790: fn test_no_ignore_parent() {
  L822: fn test_no_ignore_parent_inner_git() {
  L862: fn test_custom_ignore_precedence() {
  L888: fn test_respect_ignore_files() {
  L944: fn test_no_ignore_vcs() {
  L961: fn test_no_ignore_vcs_child_dir() {
  L977: fn test_custom_ignore_files() {
  L996: fn test_no_ignore_aliases() {
  L1015: fn test_global_ignore() {
  L1049: fn test_no_global_ignore(flag: &str, expected_output: &str) {
  L1074: fn test_file_system_boundaries() {
  L1155: fn test_max_depth() {
  L1368: fn test_exact_depth() {
  L1415: fn test_absolute_path() {
  L1452: fn test_implicit_absolute_path() {
  L1471: fn test_normalized_absolute_path() {
  L2526: fn create_file_with_modified<P: AsRef<Path>>(path: P, duration_in_secs: u64) {
  L2534: fn remove_symlink<P: AsRef<Path>>(path: P) {
  L2579: fn change_file_modified<P: AsRef<Path>>(path: P, iso_date: &str) {
  L2607: fn test_owner_ignore_all() {
  L2637: fn test_owner_root() {
  L2650: fn test_custom_path_separator() {
  L2802: fn test_single_and_multithreaded_execution() {
  L2811: fn test_number_parsing_errors() {
  L2858: fn test_error_if_hidden_not_set_and_pattern_starts_with_dot() {
  L2985: fn test_ignore_contain() {
  L17: static DEFAULT_DIRS: &[&str] = &["one/two/three", "one/two/three/directory_foo"];
  L19: static DEFAULT_FILES: &[&str] = &[
  L64: fn test_simple() {
  L82: static AND_EXTRA_FILES: &[&str] = &[
  L104: fn test_and_basic() {
  L131: fn test_and_empty_pattern() {
  L137: fn test_and_bad_pattern() {
  L147: fn test_and_pattern_starts_with_dash() {
  L167: fn test_and_plus_extension() {
  L200: fn test_and_plus_type() {
  L216: fn test_and_plus_glob() {
  L223: fn test_and_plus_fixed_strings() {
  L295: fn test_empty_pattern() {
  L383: fn test_pattern_with_forward_slash_is_rejected() {
  L425: fn test_and_pattern_with_forward_slash_is_rejected() {
  L496: fn test_regex_searches() {
  L574: fn test_glob_searches() {
  L648: fn test_glob_searches_with_extension() {
  L659: fn test_regex_overrides_glob() {
  L682: fn test_hidden() {
  L752: fn test_gitignore_and_fdignore() {
  L1056: fn test_follow() {
  L1139: fn test_print0() {
  L1381: fn test_prune() {
  L1491: fn test_type() {
  L1529: fn test_type_executable() {
  L1571: fn test_type_empty() {
  L1598: fn test_extension() {
  L1655: fn test_no_extension() {
  L1807: fn test_excludes() {
  L1856: fn format() {
  L1917: fn test_exec() {
  L2001: fn test_exec_multi() {
