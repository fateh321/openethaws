const Web3 = require('web3');
const fs = require('fs');
const abi = JSON.parse(fs.readFileSync('/data/ubuntu/openethaws/contract_easy/uniswap_sol_erc20.abi').toString());

// Initialization
const privKey =
   '4f3ea91012fc27131fdf2a62568725654726c04c46572c3eb00754b5455fe3e7'; // Genesis private key
const address = '0x93a88B7893FCDb130ab9209f63AB2e6854e617A1';
const web3 = new Web3('http://localhost:8540');
const contractAddress = '0x4FF947e19ab44afA198A3DdEaaeD817b4a8417FF';
const receiver = '0xdDa66C80C54c37d65B960AC8dFd2F0fDD2449B38';
const _value = 800;
// Contract Tx
const erc20 = new web3.eth.Contract(abi, contractAddress);
const encoded = erc20.methods.transfer(receiver,_value).encodeABI();
// erc20.methods.transfer(receiver,_value).call();
// const encoded = erc20.methods.balanceOf(address).call();
// erc20.methods.balanceOf(address).call()
// erc20.methods
//   .transfer(receiver, "100")
//   .send({ from: address }, function (err, res) {
//     if (err) {
//       console.log("An error occured", err)
//       return
//     }
//     console.log("Hash of the transaction: " + res)
//   })

const erc20tx = async () => {
   console.log(
      `Calling the transfer to ${receiver}  in contract at address ${contractAddress}`
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
erc20tx();