class UsersController < ActionController::Base
  himg_config(verbose: true, base_url: Rails.root.join("public"))

  def show
    himg_config(width: params[:w]) if params[:w]
    himg_config(gpu: true) if params[:gpu]

    @user = User.new(username: params[:id].titlecase)
  end

  def index
    himg_config(height: params[:h]) if params[:h]

    respond_to do |format|
      format.html
      format.himg { render himg: '<h1 style="text-align: center;">All Users</h1>', truncate: false }
      format.png { render himg: '<h1 style="text-align: center;">Recent Users</h1>', config: himg_config }
    end
  end
end
