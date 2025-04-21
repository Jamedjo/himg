require "rails_helper"

RSpec.describe UsersController, type: :controller do
  render_views if defined?(render_views)

  it "renders the show template with HTML format" do
    get :show, params: { id: "jamedjo" }

    expect(response).to have_http_status(:success)
    expect(response.body).to include(/>Jamedjo</)
  end

  it "renders with himg format" do
    get :show, params: { id: "jamedjo", format: :himg }

    expect(response).to have_http_status(:success)
  end

  it "pre-processes with ERB and uses that to render" do
    expect(Himg).to receive(:render).with(/Frankie/).and_call_original

    get :show, params: { id: "frankie", format: :himg }
  end
end
