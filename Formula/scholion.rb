class Scholion < Formula
  desc "Context-first version control for coding agents (CLI, av alias and daemon)"
  homepage "https://github.com/vinhqbui/scholion"
  version "0.0.2"
  license "MIT"

  # Release assets as .github/workflows/release.yml names them. The tag is release/<version>.
  # No Intel macOS build is published.
  base = "https://github.com/vinhqbui/scholion/releases/download/release/#{version}"

  on_macos do
    on_arm do
      url "#{base}/scholion-v#{version}-aarch64-apple-darwin.tar.gz"
      sha256 "61a397d35bdce5f0fd0b24fdc1f98958e961f55eff5f752d645ad72dc42cdc90"
    end
  end

  on_linux do
    on_arm do
      url "#{base}/scholion-v#{version}-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "bfe135705310f929991689a477c256b5c3aeb047dc38d27bcf321499206b3dc4"
    end
    on_intel do
      url "#{base}/scholion-v#{version}-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "68ab867aac926a37bc775e37645a4010a4b626d6c53465092c60ffbb5d36c2c0"
    end
  end

  def install
    bin.install "scholion", "av", "scholion-daemon"
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/scholion --version")
  end
end
