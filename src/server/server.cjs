const db = require('better-sqlite3')('ba.db');
var http = require('http')
var url1 = require('url');
var server = http.createServer()
server.on('request', function (req,res){
  res.setHeader('Access-Control-Allow-Origin','*')
  res.setHeader('Access-Control-Allow-Headers','*')
  res.setHeader('Access-Control-Allow-Methods','*')
  res.setHeader('Content-Type','application/json;charset=utf-8')
  res.setHeader('Access-Control-Allow-Credentials','true')
  res.writeHead(200)
  var url = req.url

  if (url === '/') {
    // res.
    res.write( JSON.stringify(db.prepare('SELECT * FROM ba').all()))
    res.end()
    }
else if (url.includes("search")) {  
    console.log(url.split("/")[2])
res.write( JSON.stringify(db.prepare('SELECT * FROM ba where id = ?').all(url.split("/")[2])))
    res.end()
  }


 

}) 
server.listen(4000, function () {
  console.log('服务器启动成功了，可以通过 http://127.0.0.1:3000/ 来进行访问')
})

