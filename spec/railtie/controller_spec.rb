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
    expect(Himg).to receive(:render).with(/Frankie/, anything).and_call_original

    get :show, params: { id: "frankie", format: :himg }
  end

  describe "default_render template lookup" do
    it "allows config options to be set at a controller level" do
      expect(Himg).to receive(:render).with(anything, hash_including(verbose: true))

      get :show, params: { id: "jamedjo", format: :png }
    end

    it "allows config options to be set within an action" do
      expect(Himg).to receive(:render).with(anything, hash_including(width: '444'))

      get :show, params: { id: "jamedjo", w: '444', format: :png }
    end
  end

  describe "manual render call" do
    it "allows config options to be sent directly through render" do
      expect(Himg).to receive(:render).with(anything, hash_including(truncate: false))

      get :index, params: { format: :himg }
    end

    it "allows config options to be set at a controller level" do
      expect(Himg).to receive(:render).with(anything, hash_including(verbose: true))

      get :index, params: { format: :png }
    end

    it "allows config options to be set within an action" do
      expect(Himg).to receive(:render).with(anything, hash_including(height: '444'))

      get :index, params: { format: :png, h: '444' }
    end
  end
end
