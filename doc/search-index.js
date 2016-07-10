var searchIndex = {};
searchIndex["ayumi"] = {"doc":"Bindings to *AY-3-8910* and *YM2149* emulation library\n[ayumi](https://github.com/pacmancoder/ayumi-lib)","items":[[3,"ToneDescriptor","ayumi","Represents tone channel control structure",null,null],[3,"NoiseDescriptor","","Represents noise channel control structure",null,null],[3,"EnvelopeDescriptor","","Represents envelope control structure",null,null],[3,"StereoSample","","Represents stereo sample",null,null],[12,"right","","right channel sample",0,null],[12,"left","","left cahnnel sample",0,null],[3,"Ayumi","","Main library structure\nuse `new` function for instance creation",null,null],[4,"ChipType","","Emulated chip type",null,null],[13,"AY","","AY-3-8910",1,null],[13,"YM","","YM2149",1,null],[4,"ToneChannel","","Tone channel id",null,null],[13,"A","","Channel A",2,null],[13,"B","","Channel B",2,null],[13,"C","","Channel C",2,null],[11,"period","","Changes period of current tone channel\n# Panics\nWhen `period` &lt; 0",3,{"inputs":[{"name":"tonedescriptor"},{"name":"i32"}],"output":{"name":"self"}}],[11,"pan","","Changes pan of current tone channel\n# Arguments\n- `pan` - value in range 0...1, which represents pan of channel\n- `equal_power` - flag, which used to enable &quot;equal_power&quot; panning",3,{"inputs":[{"name":"tonedescriptor"},{"name":"f64"},{"name":"bool"}],"output":{"name":"self"}}],[11,"volume","","Changes volume of tone channel\n# Arguments\n- `volume` - volume value of cahnnel in range [0...15]\n# Panics\nWhen value is bigger than 15",3,{"inputs":[{"name":"tonedescriptor"},{"name":"u8"}],"output":{"name":"self"}}],[11,"mixer","","Changes mixer flags of channel, use it to enable/disable sources mixing\n# Arguments\n- `tone` - tone enable flag\n- `noise` - noise enable flag\n- `envelope` - envelope enable flag",3,{"inputs":[{"name":"tonedescriptor"},{"name":"bool"},{"name":"bool"},{"name":"bool"}],"output":{"name":"self"}}],[11,"period","","Changes period of noise channel\n# Panics\nWhen `period` &lt; 0\n# Arguments\n- `period` - noise period",4,{"inputs":[{"name":"noisedescriptor"},{"name":"i32"}],"output":{"name":"self"}}],[11,"period","","Changes period of envelope\n# Panics\nWhen `period` &lt; 0\n# Arguments\n- `period` - period of envelope",5,{"inputs":[{"name":"envelopedescriptor"},{"name":"i32"}],"output":{"name":"self"}}],[11,"shape","","Changes envelope shape\n# Panics\nWhen `shape` is bigger than 15 (0x0F)\n# Arguments\n- `shape` - value in range [0...15] which represents shape of envelope",5,{"inputs":[{"name":"envelopedescriptor"},{"name":"u8"}],"output":{"name":"self"}}],[11,"new","","Constructs new Ayumi instance",6,{"inputs":[{"name":"chiptype"},{"name":"f64"},{"name":"i32"}],"output":{"name":"ayumi"}}],[11,"tone","","Returns tone channel descriptor of type `ToneDescriptor` for configuring it.",6,{"inputs":[{"name":"ayumi"},{"name":"tonechannel"}],"output":{"name":"tonedescriptor"}}],[11,"noise","","Returns noise channel descriptor of type `NoiseDescriptor` for configuring it.",6,{"inputs":[{"name":"ayumi"}],"output":{"name":"noisedescriptor"}}],[11,"envelope","","Returns envelope descriptor of type `EnvelopeDescriptor` for configuring it.",6,{"inputs":[{"name":"ayumi"}],"output":{"name":"envelopedescriptor"}}],[11,"process","","Renders next sample",6,{"inputs":[{"name":"ayumi"}],"output":{"name":"self"}}],[11,"remove_dc","","Removes the DC offset from the current sample",6,{"inputs":[{"name":"ayumi"}],"output":{"name":"self"}}],[11,"sample","","Returns sound sample of type StereoSample",6,{"inputs":[{"name":"ayumi"}],"output":{"name":"stereosample"}}]],"paths":[[3,"StereoSample"],[4,"ChipType"],[4,"ToneChannel"],[3,"ToneDescriptor"],[3,"NoiseDescriptor"],[3,"EnvelopeDescriptor"],[3,"Ayumi"]]};
initSearch(searchIndex);
