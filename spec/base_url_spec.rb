require "spec_helper"
require "himg/base_url"

RSpec.describe Himg::BaseUrl do
  context "with a nil value" do
  end

  context "with a path" do
    it "returns nil for blank path" do
      expect(Himg::BaseUrl.new("").to_s).to eq(nil)
      expect(Himg::BaseUrl.new(" ").to_s).to eq(nil)
    end

    it "returns a file:// style URI" do
      expect(Himg::BaseUrl.new("/path/to/safe/folder").to_s).to eq("file:///path/to/safe/folder/")
    end
  end

  context "with a Pathname" do
    it "returns nil for blank path" do
      expect(Himg::BaseUrl.new(Pathname.new("")).to_s).to eq(nil)
      expect(Himg::BaseUrl.new(Pathname.new(" ")).to_s).to eq(nil)
    end

    it "returns a file:// style URI" do
      expect(Himg::BaseUrl.new(Pathname.new("/path/to/safe/folder")).to_s).to eq("file:///path/to/safe/folder/")
    end
  end

  context "with a file:// URI" do
    it "rejects blank input" do
      expect { Himg::BaseUrl.new("file://") }.to raise_error(Himg::Error)
    end

    it "returns a file:// style URI" do
      expect(Himg::BaseUrl.new("file:///path/to/safe/folder").to_s).to eq("file:///path/to/safe/folder/")
    end
  end

  context "with an http:// URI" do
    it "rejects blank input" do
      expect { Himg::BaseUrl.new("http://") }.to raise_error(Himg::Error)
    end

    it "returns a http:// style URI" do
      expect(Himg::BaseUrl.new("http://mydomain.com/assets").to_s).to eq("http://mydomain.com/assets/")
      expect(Himg::BaseUrl.new("https://mydomain.com/assets").to_s).to eq("https://mydomain.com/assets/")
    end
  end
end
