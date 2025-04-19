# frozen_string_literal: true

RSpec.describe Himg do
  it "has a version number" do
    expect(Himg::VERSION).not_to be nil
  end

  it "converts HTML to an Image" do
    expect(false).to eq(true)
  end

  it "can call into Rust" do
    result = Himg.hello("earth")

    expect(result).to eq("Hello earth, from Rust!")

  end
end
