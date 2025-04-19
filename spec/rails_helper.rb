# frozen_string_literal: true

ENV['RAILS_ENV'] ||= 'test'


require_relative "./spec_helper"
require_relative "./dummy/config/application"
require "himg/railtie"

Rails.application.initialize!
