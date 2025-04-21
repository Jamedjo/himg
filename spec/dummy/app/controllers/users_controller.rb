class UsersController < ActionController::Base
  def show
    @username = params[:id].titlecase
  end
end
