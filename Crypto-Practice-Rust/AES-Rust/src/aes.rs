///全局静态变量
///一个二维数组类型
///AES的S盒
static AES_SBOX:[[u8;16];16]=[ [0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76],
    [0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0],
    [0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15],
    [0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75],
    [0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84],
    [0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf],
    [0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8],
    [0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2],
    [0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73],
    [0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb],
    [0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79],
    [0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08],
    [0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a],
    [0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e],
    [0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf],
    [0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16] ];

///AES的逆S盒
static INVERSE_AES_SBOX: [[u8;16];16] = [ [0x52, 0x09, 0x6a, 0xd5, 0x30, 0x36, 0xa5, 0x38, 0xbf, 0x40, 0xa3, 0x9e, 0x81, 0xf3, 0xd7, 0xfb],
    [0x7c, 0xe3, 0x39, 0x82, 0x9b, 0x2f, 0xff, 0x87, 0x34, 0x8e, 0x43, 0x44, 0xc4, 0xde, 0xe9, 0xcb],
    [0x54, 0x7b, 0x94, 0x32, 0xa6, 0xc2, 0x23, 0x3d, 0xee, 0x4c, 0x95, 0x0b, 0x42, 0xfa, 0xc3, 0x4e],
    [0x08, 0x2e, 0xa1, 0x66, 0x28, 0xd9, 0x24, 0xb2, 0x76, 0x5b, 0xa2, 0x49, 0x6d, 0x8b, 0xd1, 0x25],
    [0x72, 0xf8, 0xf6, 0x64, 0x86, 0x68, 0x98, 0x16, 0xd4, 0xa4, 0x5c, 0xcc, 0x5d, 0x65, 0xb6, 0x92],
    [0x6c, 0x70, 0x48, 0x50, 0xfd, 0xed, 0xb9, 0xda, 0x5e, 0x15, 0x46, 0x57, 0xa7, 0x8d, 0x9d, 0x84],
    [0x90, 0xd8, 0xab, 0x00, 0x8c, 0xbc, 0xd3, 0x0a, 0xf7, 0xe4, 0x58, 0x05, 0xb8, 0xb3, 0x45, 0x06],
    [0xd0, 0x2c, 0x1e, 0x8f, 0xca, 0x3f, 0x0f, 0x02, 0xc1, 0xaf, 0xbd, 0x03, 0x01, 0x13, 0x8a, 0x6b],
    [0x3a, 0x91, 0x11, 0x41, 0x4f, 0x67, 0xdc, 0xea, 0x97, 0xf2, 0xcf, 0xce, 0xf0, 0xb4, 0xe6, 0x73],
    [0x96, 0xac, 0x74, 0x22, 0xe7, 0xad, 0x35, 0x85, 0xe2, 0xf9, 0x37, 0xe8, 0x1c, 0x75, 0xdf, 0x6e],
    [0x47, 0xf1, 0x1a, 0x71, 0x1d, 0x29, 0xc5, 0x89, 0x6f, 0xb7, 0x62, 0x0e, 0xaa, 0x18, 0xbe, 0x1b],
    [0xfc, 0x56, 0x3e, 0x4b, 0xc6, 0xd2, 0x79, 0x20, 0x9a, 0xdb, 0xc0, 0xfe, 0x78, 0xcd, 0x5a, 0xf4],
    [0x1f, 0xdd, 0xa8, 0x33, 0x88, 0x07, 0xc7, 0x31, 0xb1, 0x12, 0x10, 0x59, 0x27, 0x80, 0xec, 0x5f],
    [0x60, 0x51, 0x7f, 0xa9, 0x19, 0xb5, 0x4a, 0x0d, 0x2d, 0xe5, 0x7a, 0x9f, 0x93, 0xc9, 0x9c, 0xef],
    [0xa0, 0xe0, 0x3b, 0x4d, 0xae, 0x2a, 0xf5, 0xb0, 0xc8, 0xeb, 0xbb, 0x3c, 0x83, 0x53, 0x99, 0x61],
    [0x17, 0x2b, 0x04, 0x7e, 0xba, 0x77, 0xd6, 0x26, 0xe1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0c, 0x7d] ];

///轮常量是一个字(4个字节),这个字的右边3个字节总为0
//Rcon[j]=(RC[j],0,0,0)
static RC:[u8;11]=[0x00, 0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1B, 0x36];

pub struct AES128{
    ///拓展密钥
    /// 内层为4个字节,外层为44个
    expanded_key:[[u8;4];44],
    ///加密
    pub encrypt: fn(&AES128,&[u8])->Vec<u8>, //函数
    //解密
    pub decrypt:fn(&AES128,&[u8])->Vec<u8>,
    encrypt_block:fn(&AES128,&[u8;16])->[u8;16],
    decrypt_block:fn(&AES128,&[u8;16])->[u8;16],
}

impl AES128{
    //初始化
    pub fn new_from_str(key:&str)->AES128 {
        let key_bytes = key.as_bytes(); //把&str转换成&[u8]
        if key_bytes.len() != 16 { //明文128位
            panic!("明文需要是16个字节长");
        }
        Self {
            expanded_key: key_schedule_aes128(&clone_into_array(key_bytes)),
            encrypt: encrypt_aes128,
            decrypt: decrypt_aes128,
            encrypt_block: encrypt_block_aes128,
            decrypt_block: decrypt_block_aes128,
        }
    }

    pub fn new(key:&[u8;16])->Self{
        Self{
            expanded_key:key_schedule_aes128(key),
            encrypt:encrypt_aes128,
            decrypt:decrypt_aes128,
            encrypt_block:encrypt_block_aes128,
            decrypt_block:decrypt_block_aes128,
        }
    }
}

//把一个切片&[T]复制到一个固定长度的数组容器Array中
//比如将动态长度的&[T]转换成固定长度[u8;16]
fn clone_into_array<A,T>(slice:&[T])->A
where 
 A:Default+AsMut<[T]>, //保证能把&mut A转成&mut [T]
 T:Clone,
{
    let mut a=A::default(); //初始化一个A:[0;16]
    a.as_mut().clone_from_slice(slice);
    //as_mut()把&mut a转成&mut [T](将数组a转换为可变切片&mut [T])
    //只有&[T]才有clone_from_slice这个方法,clone_from_slice(slice)把slice的元素逐个复制到a中
    //这里要求slice.len()==A.len()
    a
}

//生成拓展密钥
fn key_schedule_aes128(key_bytes:&[u8;16])->[[u8;4];44]{
    let mut original_key=[[0u8;4];4]; //初始密钥128位,一共4个,每个4字节
    let mut expanded_key=[[0u8;4];44];
    let N =4;// Nk=128/32=4
    for i in 0..16{
        original_key[i/4][i%4]=key_bytes[i];
        //original_key[0]=[k[0],k[1],k[2],k[3]] W[0]
        //original_key[1]=[k[4],k[5],k[6],k[7]] W[1]
        //original_key[2]=[k[8],k[9],k[10],k[11]] W[2]
        //original_key[0]=[k[12],k[13],k[14],k[15]] W[3]
    }

    for i in 0..44{ //11 轮 
        if i <N{ //第一轮
            expanded_key[i]=original_key[i];
        }
        else if i>=N&& i%N==0{ //n是4的倍数
            let mut rcon=[0u8;4]; //rcon是一个字，4个字节
            rcon[0]=RC[i/N]; //第一个字节不为0，其他三个字节都为0
            expanded_key[i]=xor_words(&xor_words(&expanded_key[i-N],&sub_word((&rot_word(&expanded_key[i-1])))),&rcon) //xor_words返回[u8;4]4个字节
            //4的倍数 W[n]=W[n-4]^SubWord(RotWord(W[n-1))^rcon[n/4]
        }
        else{
            expanded_key[i]=xor_words(&expanded_key[i-N],&expanded_key[i-1]);
        }
    }
    expanded_key
}

//S盒字节代换
fn substitute(byte:u8,encryption:bool)->u8{ //返回一个字节
    let upper_nibble:usize; //高位
    let lower_nibble:usize;
    upper_nibble=((byte>>4)&0xF).into(); //u8类型转换为usize
    lower_nibble=((byte)&0xF).into();
    if encryption==true{
        AES_SBOX[upper_nibble][lower_nibble]
    }
    else{
        INVERSE_AES_SBOX[upper_nibble][lower_nibble]
    }
}

//循环左移一个字节
fn rot_word(word:&[u8;4])->[u8;4]{
    let mut result=[0u8;4];
    for i in 0..4{
        result[i]=word[(i+1)%4];
    }
    result
}
//利用S盒对输入的每个字节进行字节代换
fn sub_word(word:&[u8;4])->[u8;4]{
    let mut result=[0u8;4];
    for i in 0..4{
        result[i]=substitute(word[i],true);
    }
    result
}

//异或操作
fn xor_words(word1:&[u8;4],word2:&[u8;4])->[u8;4]{
    let mut result=[0u8;4];
    for i in 0..4{
        result[i]=word1[i]^word2[i];
    }
    result
}

//轮密钥异或
//一个二维数组，每个是一个字节
fn add_round_key(state:&mut [[u8;4];4],key:&[[u8;4];4]){
    for i in 0..4{ //i代表行
        for j in 0..4{ //j代表列
            state[i][j]=state[i][j]^key[j][i];
        }
    }
}

//状态矩阵的每一个字节进行S盒字节替换
fn sub_bytes(state:&mut [[u8;4];4]){
    for i in 0..4{
        for j in 0..4{
            state[i][j]=substitute(state[i][j],true);
        }
    }
}

//逆S盒
fn inv_sub_bytes(state:&mut [[u8;4];4]) {
    for i in 0..4{
        for j in 0..4{
            state[i][j]=substitute(state[i][j],false);
        }
    }
}
//行移位
fn shift_rows(state:&mut [[u8;4];4]){ //可以直接用内置rotate_left[i]循环左移一位
  //从第1行开始（第0行不需要移位）
    for i in 1..4{
        let temp=state[i]; //暂存第1..3行的4个字节

        //公式:新位置[j]=原位置[(j+移位量)%长度]
        for j in 0..4{
            state[i][j]=temp[(j+i)%4];
        }
    }
}

//逆行移位
fn inv_shift_rows(state:&mut [[u8;4];4]){
    //从第1行开始（第0行不需要移位）
    for i in 1..4{
        let temp=state[i]; //暂存第1..3行的每一行4个字节

        for j in 0..4{
            state[i][j]=temp[(j+4-i)%4];
        }
    }
}

fn galois_multiplication(ap:u8,bp:u8)->u8{ //ap是被乘数，bp是乘数
    let mut temp=[0u8;8]; //8个字节，每个字节表示*(1..8)的结果
    let mut result=0u8;
    temp[0]=ap;
    for i in 1..8{
        if temp[i-1]>=0x80{ //如果最高位为1
            temp[i]=(temp[i-1]<<1)^0x1b; //异或0x1b
        }
        else
        {
            temp[i]=temp[i-1]<<1; //否则只左移一位
        }

    }
    for i in 0..8{
        if (((bp>>i)&0x01)==1) {
            result^=temp[i];
        }
    }
    result
}
//列混合
fn mix_columns(state:&mut [[u8;4];4]){
    for i in 0..4{ //列
        let mut temp=[0u8;4]; //4个字节
        for j in 0..4{ //行
            temp[j]=state[j][i]; //列优先
        }
        //temp[0]=state[0][0]
        //temp[1]=state[1][0]
        state[0][i]=galois_multiplication(temp[0],2)^galois_multiplication(temp[1],3)^galois_multiplication(temp[2],1)^galois_multiplication(temp[3],1);
        state[1][i]=galois_multiplication(temp[0],1)^galois_multiplication(temp[1],2)^galois_multiplication(temp[2],3)^galois_multiplication(temp[3],1);
        state[2][i]=galois_multiplication(temp[0],1)^galois_multiplication(temp[1],1)^galois_multiplication(temp[2],2)^galois_multiplication(temp[3],3);
        state[3][i]=galois_multiplication(temp[0],3)^galois_multiplication(temp[1],1)^galois_multiplication(temp[2],1)^galois_multiplication(temp[3],2);
    }
}

//逆裂混合
fn inv_mix_columns(state:&mut [[u8;4];4]){
    for i in 0..4{
        let mut temp=[0u8;4];
        for j in 0..4{
            temp[j]=state[j][i];
        }
        state[0][i]=galois_multiplication(temp[0],14)^galois_multiplication(temp[1],11)^galois_multiplication(temp[2],13)^galois_multiplication(temp[3],9);
        state[1][i]=galois_multiplication(temp[0],9)^galois_multiplication(temp[1],14)^galois_multiplication(temp[2],11)^galois_multiplication(temp[3],13);
        state[2][i]=galois_multiplication(temp[0],13)^galois_multiplication(temp[1],9)^galois_multiplication(temp[2],14)^galois_multiplication(temp[3],11);
        state[3][i]=galois_multiplication(temp[0],11)^galois_multiplication(temp[1],13)^galois_multiplication(temp[2],9)^galois_multiplication(temp[3],14);
    }
}

//加密
fn encrypt_aes128(aes128:&AES128,bytes:&[u8])->Vec<u8>{
    if bytes.len()%16!=0 { //不是16的倍数
        panic!("输入不是16个字节!");
    }

    let mut result=vec![0u8;bytes.len()];

    for i in 0..bytes.len()/16{ //16个字节密钥为一个block
        let mut block=[0u8;16];
        for j in 0..16{
            block[j]=bytes[i*16+j];
        }
        block=encrypt_block_aes128(aes128,&block);
        for j in 0..16{
            result[i*16+j]=block[j];
        }
    }
    result
}

fn encrypt_block_aes128(aes128: &AES128, bytes: &[u8; 16]) -> [u8; 16] {
    let mut result = [0u8; 16];
    let mut state = [[0u8; 4]; 4]; // 状态矩阵

    // 初始填充
    for i in 0..16 {
        state[i % 4][i / 4] = bytes[i];
    }

    // 打印初始状态
    debug_print_state("Init", "Input", &state);

    // --- Round 0 (Initial Round) ---
    add_round_key(&mut state, &clone_into_array(&aes128.expanded_key[0..4]));
    debug_print_state("0", "After AddRoundKey", &state);

    // --- Round 1 to 9 (Main Rounds) ---
    for i in 1..10 {
        let round_str = i.to_string();

        sub_bytes(&mut state);
        debug_print_state(&round_str, "After SubBytes", &state);

        shift_rows(&mut state);
        debug_print_state(&round_str, "After ShiftRows", &state);

        mix_columns(&mut state);
        debug_print_state(&round_str, "After MixColumns", &state);

        add_round_key(&mut state, &clone_into_array(&aes128.expanded_key[i * 4..(i + 1) * 4]));
        debug_print_state(&round_str, "After AddRoundKey", &state);
    }

    // --- Round 10 (Final Round) ---
    sub_bytes(&mut state);
    debug_print_state("10", "After SubBytes", &state);

    shift_rows(&mut state);
    debug_print_state("10", "After ShiftRows", &state);

    add_round_key(&mut state, &clone_into_array(&aes128.expanded_key[40..44]));
    debug_print_state("10", "After AddRoundKey (Cipher)", &state);

    // 输出转换
    for i in 0..4 {
        for j in 0..4 {
            result[4 * j + i] = state[i][j]; // 转化为一维数组
        }
    }
    result
}

fn decrypt_aes128(aes128:&AES128,bytes:&[u8])->Vec<u8>{
    if bytes.len()%16!=0{
        panic!("输入不是16个字节");
    }
    let mut result=vec![0u8;bytes.len()];
    for i in 0..bytes.len()/16{
        let mut block=[0u8;16];
        for j in 0..16{
            block[j]=bytes[i*16+j];
        }
        block=decrypt_block_aes128(aes128,&block);
        for j in 0..16{
            result[i*16+j]=block[j];
        }
    }
    result
}
fn decrypt_block_aes128(aes128: &AES128,bytes:&[u8;16])->[u8;16]{
    let mut result=[0u8;16];
    let mut state=[[0u8;4];4];
    for i in 0..16{
        state[i%4][i/4]=bytes[i];
    }
    add_round_key(&mut state,&clone_into_array(&aes128.expanded_key[40..44]));
    inv_shift_rows(&mut state);
    inv_sub_bytes(&mut state);

    for i in (1..10).rev(){
        add_round_key(&mut state,&clone_into_array(&aes128.expanded_key[i*4..(i+1)*4]));
        inv_mix_columns(&mut state);
        inv_shift_rows(&mut state);
        inv_sub_bytes(&mut state);
    }

    //最后一轮
    add_round_key(&mut state,&clone_into_array(&aes128.expanded_key[0..4]));

    for i in 0..4{
        for j in 0..4{
            result[4*j+i]=state[i][j];
        }
    }
    result
}

// 辅助调试函数：打印状态矩阵
// 格式化输出为 4x4 的十六进制矩阵
fn debug_print_state(round: &str, step: &str, state: &[[u8; 4]; 4]) {
    println!("--- Round {}: {} ---", round, step);
    for i in 0..4 {
        print!("Row {}: ", i);
        for j in 0..4 {
            print!("{:02X} ", state[i][j]); // :02X 表示由0补齐的两位大写16进制
        }
        println!();
    }
    println!(); // 空一行方便阅读
}

#[cfg(test)]
mod tests{
    use super::AES128; //导入父模块的AES128和其他定义
    #[test]
    fn run_test() {
        println!("开始加密: ");
        let mut plain=[
            0x32, 0x43, 0xF6, 0xA8,
            0x88, 0x5A, 0x30, 0x8D,
            0x31, 0x31, 0x98, 0xA2,
            0xE0, 0x37, 0x07, 0x34];
        let key=[
            0x2b, 0x7e, 0x15, 0x16,
            0x28, 0xae, 0xd2, 0xa6,
            0xab, 0xf7, 0x15, 0x88,
            0x09, 0xcf, 0x4f, 0x3c
        ];
        let aes= AES128::new(&key);
        let encrypt_result=(aes.encrypt)(&aes,&mut plain);
        let expect_cipher= [
            0x39,0x25,0x084,0x1D,
            0x02,0xDC,0x09,0xFB,
            0xDC,0x11,0x85,0x97,
            0x19,0x6A,0x0B,0x32
        ];
        assert_eq!(encrypt_result,expect_cipher);

    }
}
