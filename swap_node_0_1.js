const csv = require('csv-parser');
// var csv = require('fast-csv');
const Web3 = require('web3');
const fs = require('fs');
const path = require("path");
const abi=JSON.parse(fs.readFileSync(path.resolve(__dirname, "contract_uniswap/router/router_sol_UniswapV2Router02.abi")).toString());

const web3 = new Web3('http://localhost:8540');
const tokenAddress1 = '0x4FF947e19ab44afA198A3DdEaaeD817b4a8417FF';
const tokenAddress2 = '0xdDa66C80C54c37d65B960AC8dFd2F0fDD2449B38';

const contractAddress1 = '0x5bc532C8910EA2934a92A22d5dF3c868C91C9631';
const contractAddress2 = '0x7214a59d65AB044392D6f049058bc572C6195D72';

var queryParameter = ()=> new Promise( resolve =>{
	var keys = [];
	fs.createReadStream('out.csv')
	  .pipe(csv())
	  .on('data', row => {
	    keys.push(row);
	    // console.log(row);
	  })
	  .on('end',()=>{
          resolve(keys)
      })
})
var keys = [];
queryParameter().then((res)=>
	{keys = res;
	console.log("fuck you");
	let len = keys.length;
	console.log(len);
	batch = 1000;
	for (let i = 4*batch; i < 5*batch; i++) {

		random_1 = Math.floor(Math.random() * 2); 
		// if (i%2 == 0){
		if (random_1%2 == 0){

		const _amountIn = 2*i+4;
		var _amountOutMin;
		var _path;
		random = Math.floor(Math.random() * 2); 
		// if (i%2 == 0){
		if (random%2 == 0){
			_path = [tokenAddress2, tokenAddress1];
			_amountOutMin = Math.floor((_amountIn/2)*0.97);
		}else{
			_path = [tokenAddress1, tokenAddress2]
			_amountOutMin = Math.floor((_amountIn*2)*0.97);
		};
		
		const _to = '0x65e154ef9a2967e922936415bb0e2204be87b64c';
		const _deadline = 1234567891234567;
		// Initialization
		const privKey = keys[i].Privkey; // Genesis private key
		const address = keys[i].PubKey;
		// Contract Tx
		var contractAddress;
		var rand = Math.floor(Math.random() * 2); 
		// if (i%2 == 0){
		if (rand%2 == 0){
			contractAddress = contractAddress1;
		}else{
			contractAddress = contractAddress2;
		}
		const router = new web3.eth.Contract(abi, contractAddress);
		const encoded = router.methods.swapExactTokensForTokens(_amountIn, _amountOutMin, _path, _to, _deadline).encodeABI();

		const routertx = async () => {
		   console.log(
		      `swapping ${_amountIn} tokens for ${_amountOutMin} in contract at address ${contractAddress}`
		   );
		   const createTransaction = await web3.eth.accounts.signTransaction(
		      {
		         from: address,
		         to: contractAddress,
		         data: encoded,
		         gas: '429496',
		      },
		      privKey
		   );
		const createReceipt = await web3.eth.sendSignedTransaction(
		      createTransaction.rawTransaction
		   );
		   console.log(`Tx successfull with hash: ${createReceipt.transactionHash}`);
		};
		routertx();
		
		}else{

		const _amountOut = i+1;
		var _amountInMax ;
		var _path; 
		if (i%2 == 0){
			_path = [tokenAddress2, tokenAddress1];
			_amountInMax = Math.ceil((_amountOut*2+1)*1.03);
		}else{
			_path = [tokenAddress1, tokenAddress2];
			_amountInMax = Math.ceil((_amountOut/2+1)*1.03);
		};
		
		const _to = '0x65e154ef9a2967e922936415bb0e2204be87b64c';
		const _deadline = 1234567891234567;
		// Initialization
		const privKey = keys[i].Privkey; // Genesis private key
		const address = keys[i].PubKey;
		// Contract Tx
		var contractAddress;
		var rand = Math.floor(Math.random() * 2); 
		// if (i%2 == 0){
		if (rand%2 == 0){
			contractAddress = contractAddress1;
		}else{
			contractAddress = contractAddress2;
		}
		const router = new web3.eth.Contract(abi, contractAddress);
		const encoded = router.methods.swapTokensForExactTokens(_amountOut, _amountInMax, _path, _to, _deadline).encodeABI();

		const routertx = async () => {
		   console.log(
		      `swapping ${_amountOut} tokens in contract at address ${contractAddress}`
		   );
		   const createTransaction = await web3.eth.accounts.signTransaction(
		      {
		         from: address,
		         to: contractAddress,
		         data: encoded,
		         gas: '429496',
		      },
		      privKey
		   );
		const createReceipt = await web3.eth.sendSignedTransaction(
		      createTransaction.rawTransaction
		   );
		   console.log(`Tx successfull with hash: ${createReceipt.transactionHash}`);
		};
		routertx();

			
		};
		



	}

	}
)


