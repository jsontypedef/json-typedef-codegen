using System;
using System.Collections.Generic;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;

namespace Jtd.JtdCodegenDemo
{
    /// <summary>
    /// Our pre-GDPR do-not-track settings
    /// </summary>
    
    public class PreferencesDoNotTrackV0 : PreferencesDoNotTrack
    {

        [JsonProperty("version")]
        public const string Version = "v0";


        /// <value>
        /// An all-or-nothing do-not-track setting
        /// </value>
        [JsonProperty("do_not_track")]
        public bool DoNotTrack { get; set; }





    }
}
