require "rails_helper"

RSpec.describe "Himg Railtie Requests", type: :request do
  before do
    allow(Himg).to receive(:render).and_return("MOCK_PNG_DATA".unpack("C*"))
  end

  describe "GET /users/jamedjo" do
    it "renders HTML correctly" do
      get "/users/jamedjo"

      expect(response).to have_http_status(:success)
      expect(response.content_type).to include("text/html")
      expect(response.body).to include(/>Jamedjo</)
    end

    it "renders himg format correctly" do
      get "/users/jamedjo.himg"
      expect(response).to have_http_status(:success)
      expect(response.content_type).to include("image/png")
      expect(response.body).to eq("MOCK_PNG_DATA")
    end

    it "renders png format correctly" do
      get "/users/jamedjo.png"
      expect(response).to have_http_status(:success)
      expect(response.content_type).to include("image/png")
      expect(response.body).to eq("MOCK_PNG_DATA")
    end
  end
end
