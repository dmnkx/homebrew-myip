class Myip < Formula
  desc "Print local and public IP addresses"
  homepage "https://github.com/dmnkx/homebrew-myip"
  version "0.1.0"
  license "MIT"

  livecheck do
    url :homepage
    strategy :github_latest
  end

  on_macos do
    on_arm do
      url "https://github.com/dmnkx/homebrew-myip/releases/download/v0.1.0/myip-0.1.0-aarch64-apple-darwin.tar.gz"
      sha256 "0000000000000000000000000000000000000000000000000000000000000000"
    end
    on_intel do
      url "https://github.com/dmnkx/homebrew-myip/releases/download/v0.1.0/myip-0.1.0-x86_64-apple-darwin.tar.gz"
      sha256 "0000000000000000000000000000000000000000000000000000000000000000"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/dmnkx/homebrew-myip/releases/download/v0.1.0/myip-0.1.0-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "0000000000000000000000000000000000000000000000000000000000000000"
    end
    on_intel do
      url "https://github.com/dmnkx/homebrew-myip/releases/download/v0.1.0/myip-0.1.0-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "0000000000000000000000000000000000000000000000000000000000000000"
    end
  end

  def install
    bin.install "myip"
  end

  test do
    assert_match(/myip \d+\.\d+\.\d+/, shell_output("#{bin}/myip --version"))
  end
end
