# Code generated by jtd-codegen for Ruby v0.2.0

module JtdCodegenE2e

  class Root
    attr_accessor :bar
    attr_accessor :baz
    attr_accessor :foo
    attr_accessor :quux

    def self.from_json(data)
      out = Root.new
      out.bar = JtdCodegenE2e::from_json(String, data["bar"])
      out.baz = JtdCodegenE2e::from_json(Array[TrueClass], data["baz"])
      out.foo = JtdCodegenE2e::from_json(TrueClass, data["foo"])
      out.quux = JtdCodegenE2e::from_json(Array[TrueClass], data["quux"])
      out
    end

    def to_json
      data = {}
      data["bar"] = JtdCodegenE2e::to_json(bar)
      data["baz"] = JtdCodegenE2e::to_json(baz)
      data["foo"] = JtdCodegenE2e::to_json(foo)
      data["quux"] = JtdCodegenE2e::to_json(quux)
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
