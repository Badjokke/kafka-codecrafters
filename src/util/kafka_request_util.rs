pub struct DescribeTopicPartitionsRequest{
    topics: Vec<Topic>,
    response_partition_limit: i32,
    cursor: Cursor,
    tag_buffer: Vec<u8>
}
struct Topic{
    name: String,
    tag_buffer: Vec<u8>
}
struct Cursor{
    topic_name: String,
    partition_index: i32,
    tag_buffer: Vec<u8>
}

pub fn parse_describe_topics_request(buf: Vec<u8>){
    println!("Parsing describe topics request: {:?}", buf);
    let mut buffer_offset = 0;
    let topic_count = (u8::from_be_bytes(buf[buffer_offset..buffer_offset+1].try_into().unwrap()) as usize)-1;
    println!("Topics: {topic_count}");
    let (topics, new_offset ) = parse_topics(&buf, topic_count, buffer_offset+1);
    buffer_offset = new_offset;
    let partition_limit = i32::from_be_bytes(buf[buffer_offset..buffer_offset + 4].try_into().unwrap());
    buffer_offset += 4;
    let c = parse_cursor(&buf,buffer_offset);
    for i in 0..topic_count{
        println!("{:?} ", topics[i].name);
    }
    println!("partition limit: {partition_limit}");
    if c.is_none(){
        println!("No cursor!")
    }
    else {
        println!("Cursor topic name {:?}", c.unwrap().topic_name);
    }
}
//ignores tag buf
fn parse_topics(buf: &Vec<u8>, topic_count: usize, mut offset: usize) -> (Vec<Topic>,usize){
    let mut topics: Vec<Topic> = Vec::with_capacity(topic_count);
    for i in 0..topic_count{
        let topic_name_len = (u8::from_be_bytes(buf[offset..offset+1].try_into().unwrap()) as usize) -1;
        offset += 1;
        let name = String::from_utf8(buf[offset..offset+topic_name_len].to_vec()).unwrap();
        topics.push(Topic{name, tag_buffer:vec![0]});
        offset += 1 + topic_name_len;
    }
    (topics, offset)
}

fn parse_cursor(buf: &Vec<u8>, mut offset: usize) -> Option<Cursor> {
    let topic_name_len = u8::from_be_bytes(buf[offset..offset+1].try_into().unwrap()) as usize;
    //cursor is nullable 0xff indicates null
    if topic_name_len == 0xff{
        return None;
    }
    offset += 1;
    let topic_name = String::from_utf8(buf[offset..offset+topic_name_len].to_vec()).unwrap();
    offset += topic_name_len;
    let partition_index = i32::from_be_bytes(buf[offset..offset+4].try_into().unwrap());
    offset += 4;
    Some(Cursor{topic_name, partition_index, tag_buffer:vec![0]})
}