class Myip < Formula
  desc "Print local and public IP addresses"
  homepage "https://github.com/dmnkx/homebrew-myip"
  license "MIT"
  url "https://github.com/dmnkx/homebrew-myip/releases/download/v0.1.0/myip-0.1.0.tar.gz"
  sha256 "fe7ca1b0cd087654bd5d00144444de331685b93deb4bdfd9c430e77a9c99ee23"
  head "https://github.com/dmnkx/homebrew-myip.git", branch: "main"


  livecheck do
    url :homepage
    strategy :github_latest
  end
  depends_on "rust" => :build

  def install
    system "cargo", "install", "--locked", *std_cargo_args
  end

  test do
    assert_match(/myip \d+\.\d+\.\d+/, shell_output("#{bin}/myip --version"))
  end
end
