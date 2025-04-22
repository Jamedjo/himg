class UsersController < ActionController::Base
  def show
    @user = User.new(username: params[:id].titlecase)
  end

  def index
    respond_to do |format|
      format.html
      format.himg { render himg: '<h1 style="text-align: center;">All Users</h1>' }
      format.png { render himg: '<h1 style="text-align: center;">Recent Users</h1>' }
    end
  end
end
