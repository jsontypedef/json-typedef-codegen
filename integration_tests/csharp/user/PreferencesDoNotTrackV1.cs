using System;
using System.Collections.Generic;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;

namespace Jtd.JtdCodegenDemo
{
    
    public class PreferencesDoNotTrackV1 : PreferencesDoNotTrack
    {

        [JsonProperty("version")]
        public const string Version = "v1";


        [JsonProperty("opt_out_channels")]
        public IList<string> OptOutChannels { get; set; }

        [JsonProperty("do_not_track")]
        public PreferencesDoNotTrackV1DoNotTrack DoNotTrack { get; set; }





    }
}
