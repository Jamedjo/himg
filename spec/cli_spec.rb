require "spec_helper"
require "himg/cli"

RSpec.describe Himg::CLI do
  describe "screenshot" do
    subject(:cli) { described_class.new }

    let(:source_path) { "ext/himg/examples/assets/github_profile_offline.html" }
    let(:destination_path) { "tmp/destination.png" }

    it "opens a source file" do
      expect(File).to receive(:open).with(source_path)

      cli.invoke(:screenshot, [source_path, destination_path], verbose: true)
    end

    context "with a http URL" do
      let(:source_path) { "http://an.example" }

      it "network fetches the content" do
        expect(URI).to receive(:open).with(source_path)

        cli.invoke(:screenshot, [source_path, destination_path], verbose: true)
      end
    end

    context "with path that does not exist" do
      let(:source_path) { "does/not/exist" }

      it "aborts instead of trying to render" do
        expect(Himg).not_to receive(:render)

        expect do
          cli.invoke(:screenshot, [source_path, destination_path])
        end.to raise_error(Errno::ENOENT)
      end
    end

    it "renders an image" do
      expect(Himg).to receive(:render).with(anything, anything)

      cli.invoke(:screenshot, [source_path, destination_path], verbose: true)
    end

    it "saves a png" do
      destination_path = "tmp/cli_spec_saves_a_#{Time.now.to_i}.png"

      cli.invoke(:screenshot, [source_path, destination_path], verbose: true)

      contents = File.read(destination_path)

      expect(contents).to start_with("\x89PNG\r\n\x1A\n")
    ensure
      File.delete(destination_path)
    end

    describe "options" do
      it "sets image width" do
        expect(Himg).to receive(:render).with(anything, hash_including(width: 5))

        cli.invoke(:screenshot, [source_path, destination_path], width: 5)
      end

      it "sets image height" do
        expect(Himg).to receive(:render).with(anything, hash_including(height: 12))

        cli.invoke(:screenshot, [source_path, destination_path], height: 12)
      end

      it "can run in verbose mode" do
        expect(Himg).to receive(:render).with(anything, hash_including(verbose: true))

        cli.invoke(:screenshot, [source_path, destination_path], verbose: true)
      end

      it "can render a full height html page" do
        expect(Himg).to receive(:render).with(anything, hash_including(truncate: false))

        cli.invoke(:screenshot, [source_path, destination_path], truncate: false)
      end

      it "respects custom fetch_timeout" do
        expect(Himg).to receive(:render).with(anything, hash_including(fetch_timeout: 5))

        cli.invoke(:screenshot, [source_path, destination_path], fetch_timeout: 5)
      end

      it "supports fractional second fetch_timeout" do
        expect(Himg).to receive(:render).with(anything, hash_including(fetch_timeout: 0.5))

        cli.invoke(:screenshot, [source_path, destination_path], fetch_timeout: 0.5)
      end

      it "respects gpu flag" do
        expect(Himg).to receive(:render).with(anything, hash_including(gpu: true))

        cli.invoke(:screenshot, [source_path, destination_path], gpu: true)
      end

      it "defaults gpu to false" do
        expect(Himg).to receive(:render).with(anything, hash_including(gpu: false))

        cli.invoke(:screenshot, [source_path, destination_path])
      end

      it "sends base_url to Himg renerer" do
        expect(Himg).to receive(:render).with(anything, hash_including(base_url: "file:///path/to/project/folder"))

        cli.invoke(:screenshot, [source_path, destination_path], base_url: "file:///path/to/project/folder")
      end

      it "defaults base_url for http(s) so relative URLs can work" do
        expect(Himg).to receive(:render).with(anything, hash_including(base_url: "https://github.com/Jamedjo/"))

        source_path = "https://github.com/Jamedjo/himg#Setup?utm_source=frankie.cool"

        cli.invoke(:screenshot, [source_path, destination_path])
      end

      it "does not overwrite base_url for http(s) URLs" do
        expect(Himg).to receive(:render).with(anything, hash_including(base_url: "https://another.url"))

        source_path = "https://github.com/Jamedjo/himg"

        cli.invoke(:screenshot, [source_path, destination_path], base_url: "https://another.url")
      end

      it "keeps directory URLs unchanged when setting base_url" do
        expect(Himg).to receive(:render).with(anything, hash_including(base_url: "https://github.com/Jamedjo/himg/"))

        source_path = "https://github.com/Jamedjo/himg/"

        cli.invoke(:screenshot, [source_path, destination_path])
      end

      it "passes http_headers to Himg renderer" do
        expect(Himg).to receive(:render).with(anything, hash_including(http_headers: {"Authorization" => "Bearer token"}))

        cli.invoke(:screenshot, [source_path, destination_path], http_headers: {"Authorization" => "Bearer token"})
      end

      it "strips whitespace from header values" do
        expect(Himg).to receive(:render).with(anything, hash_including(http_headers: {"Authorization" => "Bearer token"}))

        cli.invoke(:screenshot, [source_path, destination_path], http_headers: {"Authorization" => " Bearer token "})
      end

      it "uses http_headers when fetching http URLs" do
        source_path = "https://frankie.cool"
        headers = {"Authorization" => "Bearer token"}

        expect(URI).to receive(:open).with(source_path, headers)

        cli.invoke(:screenshot, [source_path, destination_path], http_headers: headers)
      end

      it "handles nil http_headers for http URLs" do
        source_path = "https://frankie.cool"

        expect(URI).to receive(:open).with(source_path)

        cli.invoke(:screenshot, [source_path, destination_path])
      end
    end
  end
end
