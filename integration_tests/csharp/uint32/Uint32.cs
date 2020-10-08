using System;
using System.Collections.Generic;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;

namespace Jtd.JtdCodegenDemo
{
    
    public class Uint32 
    {


        
        public uint Value { get; set; }



        private class JsonConverter : Newtonsoft.Json.JsonConverter
        {
            public override bool CanRead => true;
            public override bool CanWrite => true;

            public override bool CanConvert(System.Type objectType)
            {
                return objectType == typeof(uint);
            }

            public override object ReadJson(JsonReader reader, System.Type objectType, object existingValue, JsonSerializer serializer)
            {
                return new Uint32 { Value = serializer.Deserialize<uint>(reader) };
            }

            public override void WriteJson(JsonWriter writer, object value, JsonSerializer serializer)
            {
                serializer.Serialize(writer, ((Uint32) value).Value);
            }
        }



    }
}
