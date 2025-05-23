class UsersController < ActionController::Base
  himg_config(verbose: true)

  def show
    himg_config(width: params[:w]) if params[:w]

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
