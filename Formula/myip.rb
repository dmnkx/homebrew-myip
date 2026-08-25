class Myip < Formula
  desc "Print local and public IP addresses"
  homepage "https://github.com/dmnkx/homebrew-myip"
  version "0.1.4"
  license "MIT"

  livecheck do
    url :homepage
    strategy :github_latest
  end

  on_macos do
    on_arm do
      url "https://github.com/dmnkx/homebrew-myip/releases/download/v0.1.4/myip-0.1.4-aarch64-apple-darwin.tar.gz"
      sha256 "20f74c6db0438f60ac60fee0254885c767ce90704b959481e7146dbd7b5a34ce"
    end
    on_intel do
      url "https://github.com/dmnkx/homebrew-myip/releases/download/v0.1.4/myip-0.1.4-x86_64-apple-darwin.tar.gz"
      sha256 "687fb8c3deb926714aabd0e272f43b50d2f76ee096b27a982e9f06acb3765814"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/dmnkx/homebrew-myip/releases/download/v0.1.4/myip-0.1.4-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "c6ebde6f95310fcdfdd0a810c154243d80b4e49be233c14ee9a1e01dce829c77"
    end
    on_intel do
      url "https://github.com/dmnkx/homebrew-myip/releases/download/v0.1.4/myip-0.1.4-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "40440539c3314d7b01d2b51c8d872ab8b18747bf15d45fda5a70bb28eb973179"
    end
  end

  def install
    bin.install "myip"
  end

  test do
    assert_match(/myip \d+\.\d+\.\d+/, shell_output("#{bin}/myip --version"))
  end
end
