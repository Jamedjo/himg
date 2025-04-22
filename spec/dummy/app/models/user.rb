require 'active_model'

class User
  include ActiveModel::Model
  include ActiveModel::Attributes

  attribute :username, :string

  def tagline
    "Software Engineer"
  end
end
