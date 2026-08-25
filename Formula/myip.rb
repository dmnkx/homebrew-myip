class Myip < Formula
  desc "Print local and public IP addresses"
  homepage "https://github.com/dmnkx/homebrew-myip"
  license "MIT"
  head "https://github.com/dmnkx/homebrew-myip.git", branch: "main"

  depends_on "rust" => :build

  def install
    system "cargo", "install", "--locked", *std_cargo_args
  end

  test do
    assert_match(/myip \d+\.\d+\.\d+/, shell_output("#{bin}/myip --version"))
  end
end
