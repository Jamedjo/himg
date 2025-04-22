module Himg
  class Railtie
    module ControllerConfig
      extend ActiveSupport::Concern

      included do
        before_action :_apply_himg_config
      end

      class_methods do
        def himg_config(options = {})
          @_himg_global_config ||= {}
          @_himg_global_config.merge!(options)
        end

        def _himg_global_config
          @_himg_global_config&.dup || {}
        end
      end

      def himg_config(options = {})
        @_himg_config.merge!(options)
        @_himg_config
      end

      def _apply_himg_config
        @_himg_config = self.class._himg_global_config
      end
    end
  end
end
