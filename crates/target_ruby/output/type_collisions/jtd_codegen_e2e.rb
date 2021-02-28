# Code generated by jtd-codegen for Ruby v0.2.0

module JtdCodegenE2e

  class RootFooBar
    attr_accessor :x

    def self.from_json(data)
      out = RootFooBar.new
      out.x = JtdCodegenE2e::from_json(TrueClass, data["x"])
      out
    end

    def to_json
      data = {}
      data["x"] = JtdCodegenE2e::to_json(x)
      data
    end
  end

  class RootFoo
    attr_accessor :bar

    def self.from_json(data)
      out = RootFoo.new
      out.bar = JtdCodegenE2e::from_json(RootFooBar, data["bar"])
      out
    end

    def to_json
      data = {}
      data["bar"] = JtdCodegenE2e::to_json(bar)
      data
    end
  end

  class RootFooBar0
    attr_accessor :x

    def self.from_json(data)
      out = RootFooBar0.new
      out.x = JtdCodegenE2e::from_json(String, data["x"])
      out
    end

    def to_json
      data = {}
      data["x"] = JtdCodegenE2e::to_json(x)
      data
    end
  end

  class Root
    attr_accessor :foo
    attr_accessor :foo_bar

    def self.from_json(data)
      out = Root.new
      out.foo = JtdCodegenE2e::from_json(RootFoo, data["foo"])
      out.foo_bar = JtdCodegenE2e::from_json(RootFooBar0, data["foo_bar"])
      out
    end

    def to_json
      data = {}
      data["foo"] = JtdCodegenE2e::to_json(foo)
      data["foo_bar"] = JtdCodegenE2e::to_json(foo_bar)
      data
    end
  end

  private

  def self.from_json(type, data)
    if data.nil? || [Object, TrueClass, Integer, Float, String].include?(type)
      data
    elsif type.is_a?(Array)
      data.map { |elem| from_json(type.first, elem) }
    elsif type.is_a?(Hash)
      data.transform_values { |elem| from_json(type.values.first, elem) }
    else
      type.from_json(data)
    end
  end

  def self.to_json(data)
    if data.nil? || [TrueClass, FalseClass, Integer, Float, String].include?(data.class)
      data
    elsif data.is_a?(Array)
      data.map { |elem| to_json(elem) }
    elsif data.is_a?(Hash)
      data.transform_values { |elem| to_json(elem) }
    else
      data.to_json
    end
  end
end
