const csv = require('csv-parser');
// var csv = require('fast-csv');
const Web3 = require('web3');
const fs = require('fs');
const path = require("path");
const web3 = new Web3('http://localhost:8540');
// const web3 = new Web3('172.31.39.234:8541');

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
	const t = 4000
	for (let i = 0; i < t; i++) {
		const addressTo = '0x65e154ef9a2967e922936415bb0e2204be87b64c';
		// Initialization
		const privKey = keys[i].Privkey; // Genesis private key
		const addressFrom = keys[i].PubKey;

		const deploy = async () => {
		   console.log(
		      `Attempting to make transaction from ${addressFrom} to ${addressTo}`
		   );

		   const createTransaction = await web3.eth.accounts.signTransaction(
		      {
		         from: addressFrom,
		         to: addressTo,
		         value: web3.utils.toWei('1', 'ether'),
		         gas: '21000',
		      },
		      privKey
		   );

		   // Deploy transaction
		   const createReceipt = await web3.eth.sendSignedTransaction(
		      createTransaction.rawTransaction
		   );
		   // console.log(
		   //    `Transaction successful with hash: ${createReceipt.transactionHash}`
		   // );
		};

		deploy();



	}


	}
)


