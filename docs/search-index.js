var N=null,E="",T="t",U="u",searchIndex={};
var R=["Apollo","apollo","smallrng","configerror","configpreset","result","configscale","string","try_from","try_into","borrow_mut","to_owned","clone_into","type_id","borrow","typeid","to_string","formatter","deserialize","backtrace","option","ConfigPreset","ConfigPhrase","ConfigNote","ConfigADSR","ConfigRhythm","ConfigScale","ConfigSynth","ConfigTrack","ConfigError","ConfigRange","ConfigRhythmWeight","ConfigSynthSignal","ConfigChoice"];

searchIndex["apollo"]={"doc":R[0],"i":[[3,"Config",R[1],"The configuration for song generation",N,N],[12,"seed",E,"A default seed for the random number generator",0,N],[12,"presets",E,"A list of song configuration presets",0,N],[12,"scales",E,"A list of musical scales references by song presets",0,N],[3,R[33],E,"A list of values from which one can be randomly selected",N,N],[3,R[21],E,"A song preset configuration",N,N],[12,"id",E,"The identifier for this song preset",1,N],[12,"rhythm",E,"The rhythm configuration for this song preset",1,N],[12,"scale",E,"A list of ConfigScale tags from which one will be randomly…",1,N],[12,"tracks",E,"A list of track configurations for this song preset",1,N],[3,R[22],E,"A musical phrase configuration consisting of a series of…",N,N],[12,"length",E,"A range of values for the number of bars in the phrase",2,N],[12,"note",E,"The note configuration for this phrase",2,N],[3,R[23],E,"A musical note configuration",N,N],[12,"frequency",E,E,3,N],[12,"adsr",E,E,3,N],[3,R[24],E,"An attack, decay, sustain, release envelope configuration",N,N],[12,"amplitude",E,"A range of amplitude values from 0.0 to 1.0",4,N],[12,"attack",E,"A range of attack lengths from 0.0 to 1.0",4,N],[12,"release",E,"A range of release lengths from 0.0 to 1.0",4,N],[3,R[25],E,"A musical rhythm configuration",N,N],[12,"bpm",E,"A range of bpm (beats per minute) values",5,N],[12,"beat",E,"A range of beats per bar values (time signature upper…",5,N],[12,"unit",E,"A list of beat units from which one will be randomly…",5,N],[12,"weight",E,"The note length weighting configuration for this rhythm",5,N],[3,R[26],E,"A musical scale configuration",N,N],[12,"name",E,"The name of the musical scale configuration",6,N],[12,"tags",E,"A list of tags referenced by ConfigPresets such as \"major\"…",6,N],[12,"notes",E,"A list of pitches for the scale specified in Hz with the…",6,N],[3,R[27],E,"A synthesizer configuration",N,N],[12,"num",E,"A range of values for the number of base signals for this…",7,N],[12,"offset",E,"A range of values that the signals for this synthesizer…",7,N],[12,"signal",E,"A list of ConfigSynthSignal values from which one will be…",7,N],[3,R[28],E,"A track configuration consisting of a musical phrase and…",N,N],[12,"num",E,"A range of values for the number of tracks of this…",8,N],[12,"phrase",E,"The musical phrase configuration for this track",8,N],[12,"synth",E,"The synthesizer configuration for this track",8,N],[3,R[0],E,"An iterator that produces audio samples",N,N],[4,R[29],E,"A configuration error",N,N],[13,"UnknownPreset",E,"An unknown preset ID was specified",9,N],[13,"NoScaleTag",E,"No scale tags could be found in a preset configuration",9,N],[13,"InvalidScaleTag",E,"No scales with a specified tag could be found",9,N],[4,R[30],E,"A range of values from min to max that a value can be…",N,N],[13,"Range",E,E,10,N],[12,"min","apollo::ConfigRange","The minimum value that can be randomly generated",10,N],[12,"max",E,"The maximum value that can be randomly generated",10,N],[13,"Fixed",R[1],E,10,N],[4,R[31],E,"A note length weighting configuration for this rhythm",N,N],[13,"Shorter",E,"Increase weighting of shorter notes",11,N],[13,"Longer",E,"Increase weighting of longer notes",11,N],[4,R[32],E,"A synthesizer signal type configuration",N,N],[13,"Sine",E,"Sine wave",12,N],[13,"Saw",E,"Saw wave",12,N],[13,"Square",E,"Square wave",12,N],[11,"get",E,E,13,[[["self"],[R[2]]],[[R[20]],[T]]]],[11,"get",E,E,10,[[["self"],[R[2]]],[T]]],[11,"preset",E,"Find the ConfigPreset associated with the specified preset…",0,[[["self"],[R[7]]],[[R[3]],[R[5],[R[4],R[3]]],[R[4]]]]],[11,"scale",E,"Randomly select a ConfigScale from the specified…",0,[[["self"],[R[4]],[R[2]]],[[R[5],[R[6],R[3]]],[R[3]],[R[6]]]]],[11,"new",E,"Creates a new sample generator based on the specified…",14,[[["f32"],["config"],[R[7]],["u64"]],[["error"],[R[5],["error"]]]]],[11,"into",E,E,0,[[],[U]]],[11,"from",E,E,0,[[[T]],[T]]],[11,R[8],E,E,0,[[[U]],[R[5]]]],[11,R[9],E,E,0,[[],[R[5]]]],[11,R[14],E,E,0,[[["self"]],[T]]],[11,R[10],E,E,0,[[["self"]],[T]]],[11,R[13],E,E,0,[[["self"]],[R[15]]]],[11,"vzip",E,E,0,[[],["v"]]],[11,"into",E,E,13,[[],[U]]],[11,"from",E,E,13,[[[T]],[T]]],[11,R[11],E,E,13,[[["self"]],[T]]],[11,R[12],E,E,13,[[["self"],[T]]]],[11,R[8],E,E,13,[[[U]],[R[5]]]],[11,R[9],E,E,13,[[],[R[5]]]],[11,R[14],E,E,13,[[["self"]],[T]]],[11,R[10],E,E,13,[[["self"]],[T]]],[11,R[13],E,E,13,[[["self"]],[R[15]]]],[11,"vzip",E,E,13,[[],["v"]]],[11,"into",E,E,1,[[],[U]]],[11,"from",E,E,1,[[[T]],[T]]],[11,R[11],E,E,1,[[["self"]],[T]]],[11,R[12],E,E,1,[[["self"],[T]]]],[11,R[8],E,E,1,[[[U]],[R[5]]]],[11,R[9],E,E,1,[[],[R[5]]]],[11,R[14],E,E,1,[[["self"]],[T]]],[11,R[10],E,E,1,[[["self"]],[T]]],[11,R[13],E,E,1,[[["self"]],[R[15]]]],[11,"vzip",E,E,1,[[],["v"]]],[11,"into",E,E,2,[[],[U]]],[11,"from",E,E,2,[[[T]],[T]]],[11,R[11],E,E,2,[[["self"]],[T]]],[11,R[12],E,E,2,[[["self"],[T]]]],[11,R[8],E,E,2,[[[U]],[R[5]]]],[11,R[9],E,E,2,[[],[R[5]]]],[11,R[14],E,E,2,[[["self"]],[T]]],[11,R[10],E,E,2,[[["self"]],[T]]],[11,R[13],E,E,2,[[["self"]],[R[15]]]],[11,"vzip",E,E,2,[[],["v"]]],[11,"into",E,E,3,[[],[U]]],[11,"from",E,E,3,[[[T]],[T]]],[11,R[11],E,E,3,[[["self"]],[T]]],[11,R[12],E,E,3,[[["self"],[T]]]],[11,R[8],E,E,3,[[[U]],[R[5]]]],[11,R[9],E,E,3,[[],[R[5]]]],[11,R[14],E,E,3,[[["self"]],[T]]],[11,R[10],E,E,3,[[["self"]],[T]]],[11,R[13],E,E,3,[[["self"]],[R[15]]]],[11,"vzip",E,E,3,[[],["v"]]],[11,"into",E,E,4,[[],[U]]],[11,"from",E,E,4,[[[T]],[T]]],[11,R[11],E,E,4,[[["self"]],[T]]],[11,R[12],E,E,4,[[["self"],[T]]]],[11,R[8],E,E,4,[[[U]],[R[5]]]],[11,R[9],E,E,4,[[],[R[5]]]],[11,R[14],E,E,4,[[["self"]],[T]]],[11,R[10],E,E,4,[[["self"]],[T]]],[11,R[13],E,E,4,[[["self"]],[R[15]]]],[11,"vzip",E,E,4,[[],["v"]]],[11,"into",E,E,5,[[],[U]]],[11,"from",E,E,5,[[[T]],[T]]],[11,R[11],E,E,5,[[["self"]],[T]]],[11,R[12],E,E,5,[[["self"],[T]]]],[11,R[8],E,E,5,[[[U]],[R[5]]]],[11,R[9],E,E,5,[[],[R[5]]]],[11,R[14],E,E,5,[[["self"]],[T]]],[11,R[10],E,E,5,[[["self"]],[T]]],[11,R[13],E,E,5,[[["self"]],[R[15]]]],[11,"vzip",E,E,5,[[],["v"]]],[11,"into",E,E,6,[[],[U]]],[11,"from",E,E,6,[[[T]],[T]]],[11,R[11],E,E,6,[[["self"]],[T]]],[11,R[12],E,E,6,[[["self"],[T]]]],[11,R[8],E,E,6,[[[U]],[R[5]]]],[11,R[9],E,E,6,[[],[R[5]]]],[11,R[14],E,E,6,[[["self"]],[T]]],[11,R[10],E,E,6,[[["self"]],[T]]],[11,R[13],E,E,6,[[["self"]],[R[15]]]],[11,"vzip",E,E,6,[[],["v"]]],[11,"into",E,E,7,[[],[U]]],[11,"from",E,E,7,[[[T]],[T]]],[11,R[11],E,E,7,[[["self"]],[T]]],[11,R[12],E,E,7,[[["self"],[T]]]],[11,R[8],E,E,7,[[[U]],[R[5]]]],[11,R[9],E,E,7,[[],[R[5]]]],[11,R[14],E,E,7,[[["self"]],[T]]],[11,R[10],E,E,7,[[["self"]],[T]]],[11,R[13],E,E,7,[[["self"]],[R[15]]]],[11,"vzip",E,E,7,[[],["v"]]],[11,"into",E,E,8,[[],[U]]],[11,"from",E,E,8,[[[T]],[T]]],[11,R[11],E,E,8,[[["self"]],[T]]],[11,R[12],E,E,8,[[["self"],[T]]]],[11,R[8],E,E,8,[[[U]],[R[5]]]],[11,R[9],E,E,8,[[],[R[5]]]],[11,R[14],E,E,8,[[["self"]],[T]]],[11,R[10],E,E,8,[[["self"]],[T]]],[11,R[13],E,E,8,[[["self"]],[R[15]]]],[11,"vzip",E,E,8,[[],["v"]]],[11,"into",E,E,14,[[],[U]]],[11,R[16],E,E,14,[[["self"]],[R[7]]]],[11,"into_iter",E,E,14,[[],["i"]]],[11,"from",E,E,14,[[[T]],[T]]],[11,R[8],E,E,14,[[[U]],[R[5]]]],[11,R[9],E,E,14,[[],[R[5]]]],[11,R[14],E,E,14,[[["self"]],[T]]],[11,R[10],E,E,14,[[["self"]],[T]]],[11,R[13],E,E,14,[[["self"]],[R[15]]]],[11,"vzip",E,E,14,[[],["v"]]],[11,"into",E,E,9,[[],[U]]],[11,R[16],E,E,9,[[["self"]],[R[7]]]],[11,"from",E,E,9,[[[T]],[T]]],[11,R[11],E,E,9,[[["self"]],[T]]],[11,R[12],E,E,9,[[["self"],[T]]]],[11,R[8],E,E,9,[[[U]],[R[5]]]],[11,R[9],E,E,9,[[],[R[5]]]],[11,R[14],E,E,9,[[["self"]],[T]]],[11,R[10],E,E,9,[[["self"]],[T]]],[11,R[13],E,E,9,[[["self"]],[R[15]]]],[11,"vzip",E,E,9,[[],["v"]]],[11,"as_fail",E,E,9,[[["self"]],["fail"]]],[11,"into",E,E,10,[[],[U]]],[11,"from",E,E,10,[[[T]],[T]]],[11,R[11],E,E,10,[[["self"]],[T]]],[11,R[12],E,E,10,[[["self"],[T]]]],[11,R[8],E,E,10,[[[U]],[R[5]]]],[11,R[9],E,E,10,[[],[R[5]]]],[11,R[14],E,E,10,[[["self"]],[T]]],[11,R[10],E,E,10,[[["self"]],[T]]],[11,R[13],E,E,10,[[["self"]],[R[15]]]],[11,"vzip",E,E,10,[[],["v"]]],[11,"into",E,E,11,[[],[U]]],[11,"from",E,E,11,[[[T]],[T]]],[11,R[11],E,E,11,[[["self"]],[T]]],[11,R[12],E,E,11,[[["self"],[T]]]],[11,R[8],E,E,11,[[[U]],[R[5]]]],[11,R[9],E,E,11,[[],[R[5]]]],[11,R[14],E,E,11,[[["self"]],[T]]],[11,R[10],E,E,11,[[["self"]],[T]]],[11,R[13],E,E,11,[[["self"]],[R[15]]]],[11,"vzip",E,E,11,[[],["v"]]],[11,"into",E,E,12,[[],[U]]],[11,"from",E,E,12,[[[T]],[T]]],[11,R[11],E,E,12,[[["self"]],[T]]],[11,R[12],E,E,12,[[["self"],[T]]]],[11,R[8],E,E,12,[[[U]],[R[5]]]],[11,R[9],E,E,12,[[],[R[5]]]],[11,R[14],E,E,12,[[["self"]],[T]]],[11,R[10],E,E,12,[[["self"]],[T]]],[11,R[13],E,E,12,[[["self"]],[R[15]]]],[11,"vzip",E,E,12,[[],["v"]]],[11,"clone",E,E,13,[[["self"]],["configchoice"]]],[11,"clone",E,E,9,[[["self"]],[R[3]]]],[11,"clone",E,E,4,[[["self"]],["configadsr"]]],[11,"clone",E,E,3,[[["self"]],["confignote"]]],[11,"clone",E,E,2,[[["self"]],["configphrase"]]],[11,"clone",E,E,1,[[["self"]],[R[4]]]],[11,"clone",E,E,6,[[["self"]],[R[6]]]],[11,"clone",E,E,7,[[["self"]],["configsynth"]]],[11,"clone",E,E,12,[[["self"]],["configsynthsignal"]]],[11,"clone",E,E,10,[[["self"]],["configrange"]]],[11,"clone",E,E,5,[[["self"]],["configrhythm"]]],[11,"clone",E,E,11,[[["self"]],["configrhythmweight"]]],[11,"clone",E,E,8,[[["self"]],["configtrack"]]],[11,"next",E,E,14,[[["self"]],[["f32"],[R[20],["f32"]]]]],[11,"fmt",E,E,9,[[["self"],[R[17]]],[R[5]]]],[11,"fmt",E,E,14,[[["self"],[R[17]]],[R[5]]]],[11,"fmt",E,E,13,[[["self"],[R[17]]],[R[5]]]],[11,"fmt",E,E,9,[[["self"],[R[17]]],[R[5]]]],[11,"fmt",E,E,4,[[["self"],[R[17]]],[R[5]]]],[11,"fmt",E,E,3,[[["self"],[R[17]]],[R[5]]]],[11,"fmt",E,E,2,[[["self"],[R[17]]],[R[5]]]],[11,"fmt",E,E,1,[[["self"],[R[17]]],[R[5]]]],[11,"fmt",E,E,6,[[["self"],[R[17]]],[R[5]]]],[11,"fmt",E,E,7,[[["self"],[R[17]]],[R[5]]]],[11,"fmt",E,E,12,[[["self"],[R[17]]],[R[5]]]],[11,"fmt",E,E,10,[[["self"],[R[17]]],[R[5]]]],[11,"fmt",E,E,5,[[["self"],[R[17]]],[R[5]]]],[11,"fmt",E,E,11,[[["self"],[R[17]]],[R[5]]]],[11,"fmt",E,E,8,[[["self"],[R[17]]],[R[5]]]],[11,"fmt",E,E,0,[[["self"],[R[17]]],[R[5]]]],[11,"fmt",E,E,14,[[["self"],[R[17]]],[R[5]]]],[11,R[18],E,E,13,[[["__d"]],[R[5]]]],[11,R[18],E,E,4,[[["__d"]],[R[5]]]],[11,R[18],E,E,3,[[["__d"]],[R[5]]]],[11,R[18],E,E,2,[[["__d"]],[R[5]]]],[11,R[18],E,E,1,[[["__d"]],[R[5]]]],[11,R[18],E,E,6,[[["__d"]],[R[5]]]],[11,R[18],E,E,7,[[["__d"]],[R[5]]]],[11,R[18],E,E,12,[[["__d"]],[R[5]]]],[11,R[18],E,E,10,[[["__d"]],[R[5]]]],[11,R[18],E,E,5,[[["__d"]],[R[5]]]],[11,R[18],E,E,11,[[["__d"]],[R[5]]]],[11,R[18],E,E,8,[[["__d"]],[R[5]]]],[11,R[18],E,E,0,[[["__d"]],[R[5]]]],[11,"name",E,E,9,[[["self"]],[[R[20],["str"]],["str"]]]],[11,"cause",E,E,9,[[["self"]],[[R[20],["fail"]],["fail"]]]],[11,R[19],E,E,9,[[["self"]],[[R[19]],[R[20],[R[19]]]]]]],"p":[[3,"Config"],[3,R[21]],[3,R[22]],[3,R[23]],[3,R[24]],[3,R[25]],[3,R[26]],[3,R[27]],[3,R[28]],[4,R[29]],[4,R[30]],[4,R[31]],[4,R[32]],[3,R[33]],[3,R[0]]]};
initSearch(searchIndex);addSearchOptions(searchIndex);