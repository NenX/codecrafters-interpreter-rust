import { createServer } from 'http'

createServer((req, res) => {
    let s = '{"name":2123}'
    let b = Buffer.from(s)
    b.forEach((v,idx)=>{
        b[idx] = ~v;
    })
    res.write(b)
    res.end()
}).listen(3456)