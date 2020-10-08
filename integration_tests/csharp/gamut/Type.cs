using System;
using System.Collections.Generic;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;

namespace Jtd.JtdCodegenDemo
{
    
    public class Type 
    {


        [JsonProperty("uint8")]
        public byte Uint8 { get; set; }

        [JsonProperty("float64")]
        public double Float64 { get; set; }

        [JsonProperty("float32")]
        public float Float32 { get; set; }

        [JsonProperty("int8")]
        public sbyte Int8 { get; set; }

        [JsonProperty("boolean")]
        public bool Boolean { get; set; }

        [JsonProperty("timestamp")]
        public DateTime Timestamp { get; set; }

        [JsonProperty("int16")]
        public short Int16 { get; set; }

        [JsonProperty("uint16")]
        public ushort Uint16 { get; set; }

        [JsonProperty("string")]
        public string String { get; set; }

        [JsonProperty("uint32")]
        public uint Uint32 { get; set; }

        [JsonProperty("int32")]
        public int Int32 { get; set; }





    }
}
