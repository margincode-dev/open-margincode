class Margincode < Formula
  desc "Context-first version control for coding agents (CLI, av alias and daemon)"
  homepage "https://github.com/margincode-dev/open-margincode"
  version "0.0.4"
  license "MIT"

  # Release assets as .github/workflows/release.yml names them. The tag is release/<version>.
  # No Intel macOS build is published.
  base = "https://github.com/margincode-dev/open-margincode/releases/download/release/#{version}"

  on_macos do
    on_arm do
      url "#{base}/margincode-v#{version}-aarch64-apple-darwin.tar.gz"
      sha256 "58a10eeec6f0109cb8f80db11a0f4b7c845d5cafec09c96f49c0e2cf3cd4b0fe"
    end
  end

  on_linux do
    on_arm do
      url "#{base}/margincode-v#{version}-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "c5fff84910b140a052208cbf87b66e26310ebd903ae7a4c4e0755bd36ebf76c3"
    end
    on_intel do
      url "#{base}/margincode-v#{version}-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "f59a2da7f75384530fc6d59312b2de284b4afc2e281a9b5ad624f252f65bfe0f"
    end
  end

  def install
    bin.install "margincode", "av", "margincode-daemon"
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/margincode --version")
  end
end
