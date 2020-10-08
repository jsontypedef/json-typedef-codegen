using System;
using System.Collections.Generic;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;

namespace Jtd.JtdCodegenDemo
{
    
    public class Preferences 
    {


        [JsonProperty("do_not_track")]
        public PreferencesDoNotTrack DoNotTrack { get; set; }

        [JsonProperty("title")]
        public PreferencesTitle Title { get; set; }





    }
}
