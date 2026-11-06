class Scholion < Formula
  desc "Context-first version control for coding agents (CLI, av alias and daemon)"
  homepage "https://github.com/vinhqbui/scholion"
  version "0.0.3"
  license "MIT"

  # Release assets as .github/workflows/release.yml names them. The tag is release/<version>.
  # No Intel macOS build is published.
  base = "https://github.com/vinhqbui/scholion/releases/download/release/#{version}"

  on_macos do
    on_arm do
      url "#{base}/scholion-v#{version}-aarch64-apple-darwin.tar.gz"
      sha256 "05e7cf7df15ff76e3f1e213e17c6e85ddba7a3b0342f3dc1f0746a9171726cf1"
    end
  end

  on_linux do
    on_arm do
      url "#{base}/scholion-v#{version}-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "cefd844f04e9b404dc680b7bc0b04b0b0f736175271fefa0b476bfbe03164b78"
    end
    on_intel do
      url "#{base}/scholion-v#{version}-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "ebf09d05ae6e841f22a15a7debdcf2d4db3c0968aba24d8eb7f6ad389f3bc4f7"
    end
  end

  def install
    bin.install "scholion", "av", "scholion-daemon"
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/scholion --version")
  end
end
