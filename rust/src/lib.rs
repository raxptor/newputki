trait Env
{
	type OpDesc;
}

struct PutkFull
{

}

pub enum PipelineDesc
{
	Default,
	Specific(String)
}

pub enum InputDesc
{
	Name(String),
	Hash(u64),
	Literal(Vec<u8>)
}

struct OpDesc
{
	pub source : InputDesc,
	pub pipeline : PipelineDesc
}

struct OutputObjRef 
{
	pub op_desc : OpDesc,
	pub sub_path : u32
}

struct InputView
{

}

struct ResultContent
{
	pub data : Vec<u8>,
	pub additional : Vec<InputDesc>
}

struct CompleteResult
{
	pub source : InputDesc,
	pub view : InputView,
	pub content : ResultContent
}

pub fn create(op:OpDesc)
{

}

pub fn load_object(path:String)
{
	println!("Load object [{}]", path);
	let pd = PipelineDesc::Default;
	let od = OpDesc {
		source: InputDesc::Name(path),
		pipeline: pd
	};
	let oor = OutputObjRef {
		op_desc: od,
		sub_path: 0
	};
}

pub fn init()
{
	println!("Welcome!");
}
