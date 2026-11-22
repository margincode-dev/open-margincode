class Margincode < Formula
  desc "Context-first version control for coding agents (CLI, av alias and daemon)"
  homepage "https://github.com/margincode-dev/open-margincode"
  version "0.0.5"
  license "MIT"

  # Release assets as .github/workflows/release.yml names them. The tag is release/<version>.
  # No Intel macOS build is published.
  base = "https://github.com/margincode-dev/open-margincode/releases/download/release/#{version}"

  on_macos do
    on_arm do
      url "#{base}/margincode-v#{version}-aarch64-apple-darwin.tar.gz"
      sha256 "e5394c92de4d7b26c549792a72edbd46c1fed643a1d8e1173eaac21bc72b80ac"
    end
  end

  on_linux do
    on_arm do
      url "#{base}/margincode-v#{version}-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "dfbcdf9c23fd1996bd5b52340c2fafc3d87fda66a412dad3e58c0e2b2fd26aad"
    end
    on_intel do
      url "#{base}/margincode-v#{version}-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "963d0b53e8ffbbd4c9f8d22c6a9172f2a11efd838a48bc76d82dfc839b842499"
    end
  end

  def install
    bin.install "margincode", "av", "margincode-daemon"
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/margincode --version")
  end
end
