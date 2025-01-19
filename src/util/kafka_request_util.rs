use crate::util::byte_util::ToBytes;
pub struct DescribeTopicPartitionsRequest{
    pub topics: Vec<Topic>,
    pub response_partition_limit: i32,
    pub cursor: Option<Cursor>,
    pub tag_buffer: Vec<u8>
}
pub struct Topic{
    pub name: String,
    pub tag_buffer: Vec<u8>
}
pub struct Cursor{
    topic_name: String,
    partition_index: i32,
    tag_buffer: Vec<u8>
}

pub fn parse_describe_topics_request(buf: Vec<u8>) -> DescribeTopicPartitionsRequest{
    println!("Parsing describe topics request: {:?}", buf);
    let mut buffer_offset = 0;
    let topic_count = (u8::from_be_bytes(buf[buffer_offset..buffer_offset+1].try_into().unwrap()) as usize)-1;
    println!("Topics: {topic_count}");
    let (topics, new_offset ) = parse_topics(&buf, topic_count, buffer_offset+1);
    buffer_offset = new_offset;
    let partition_limit = i32::from_be_bytes(buf[buffer_offset..buffer_offset + 4].try_into().unwrap());
    buffer_offset += 4;
    let c = parse_cursor(&buf,buffer_offset);
    println!("partition limit: {partition_limit}");
    DescribeTopicPartitionsRequest{topics, response_partition_limit: partition_limit, cursor: c, tag_buffer:vec![0]}
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




impl ToBytes for Cursor{
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(((&self.topic_name.len()+1)as u8).to_be_bytes());
        bytes.extend_from_slice(&self.topic_name.as_bytes());
        bytes.extend(&self.partition_index.to_be_bytes());
        bytes.append(&mut vec![0]);
        bytes
    }
}