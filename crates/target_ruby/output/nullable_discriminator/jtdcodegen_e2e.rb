# Code generated by jtd-codegen for Ruby v0.1.1

require 'json'
require 'time'

module JTDCodegenE2E

  class Root0
    attr_accessor :foo

    def self.from_json_data(data)
      {
        "bar" => RootBar,
        "quux" => RootQuux,
      }[data["foo"]].from_json_data(data)
    end
  end

  class RootBar < Root0
    attr_accessor :baz

    def self.from_json_data(data)
      out = RootBar.new
      out.foo = "bar"
      out.baz = JTDCodegenE2E::from_json_data(String, data["baz"])
      out
    end

    def to_json_data
      data = { "foo" => "bar" }
      data["baz"] = JTDCodegenE2E::to_json_data(baz)
      data
    end
  end

  class RootQuux < Root0
    attr_accessor :quuz

    def self.from_json_data(data)
      out = RootQuux.new
      out.foo = "quux"
      out.quuz = JTDCodegenE2E::from_json_data(String, data["quuz"])
      out
    end

    def to_json_data
      data = { "foo" => "quux" }
      data["quuz"] = JTDCodegenE2E::to_json_data(quuz)
      data
    end
  end

  class Root
    attr_accessor :value

    def self.from_json_data(data)
      out = Root.new
      out.value = JTDCodegenE2E.from_json_data(Root0, data)
      out
    end

    def to_json_data
      JTDCodegenE2E.to_json_data(value)
    end
  end

  private

  def self.from_json_data(type, data)
    if data.nil? || [Object, TrueClass, Integer, Float, String].include?(type)
      data
    elsif type == DateTime
      DateTime.rfc3339(data)
    elsif type.is_a?(Array)
      data.map { |elem| from_json_data(type.first, elem) }
    elsif type.is_a?(Hash)
      data.transform_values { |elem| from_json_data(type.values.first, elem) }
    else
      type.from_json_data(data)
    end
  end

  def self.to_json_data(data)
    if data.nil? || [TrueClass, FalseClass, Integer, Float, String].include?(data.class)
      data
    elsif data.is_a?(DateTime)
      data.rfc3339
    elsif data.is_a?(Array)
      data.map { |elem| to_json_data(elem) }
    elsif data.is_a?(Hash)
      data.transform_values { |elem| to_json_data(elem) }
    else
      data.to_json_data
    end
  end
end
