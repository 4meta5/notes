# Token Standards

> "Although Ethereum allows developers to create absolutely any kind of application without restriction to specific feature types, and prides itself on its "lack of features", there is nevertheless a need to standardize certain very common use cases in order to allow users and applications to more easily interact with each other. This includes sending currency units, registering names, making offers on exchanges, and other similar functions. A standard typically consists of a set of function signatures for a few methods, eg. send, register, delete, providing the set of arguments and their formats in the Ethereum contract ABI language." [Vitalik's original post, which motivated ERC 20](https://github.com/ethereum/wiki/wiki/Standardized_Contract_APIs/499c882f3ec123537fc2fccd57eaa29e6032fe4a)

Because token standards formalize how we store data on the blockchain, it is important to integrate these designs into the applications motivated by Radical Markets (and enabled by blockchain). Of course, following token standards is not necessary, but it is better to build on top of this work rather than reinvent the wheel.

**Random Important Links**<br>
* [EIP Website](https://eips.ethereum.org/)
* [Ethereum Contract ABI Specification](https://solidity.readthedocs.io/en/develop/abi-spec.html#basic-design)
* [Walking Through the ERC721 Full Implementation](https://medium.com/blockchannel/walking-through-the-erc721-full-implementation-72ad72735f3c)
* [OpenZeppelin Solidity Token Contracts](https://github.com/OpenZeppelin/openzeppelin-solidity/tree/master/contracts/token)

**Token Standard Notes**<br>
* [Contract ABI Spec](#abi)
* [ERC20](#erc20)
* [ERC165](#erc165)
* [ERC721](#erc721)
* [ERC998](#erc998)

## Contract ABI Specification <a name = "abi"></a>
* [relevant documentation](https://solidity.readthedocs.io/en/develop/abi-spec.html#) <br>
The first four bytes of the call data for a function call specifies the function to be called. It is the first (left, high-order in big-endian) four bytes of the Kecccak-256 (SHA-3) hash of the signature of the function. 

> The signature is defined as the canonical expression of the basic prototype without the data location specifier, i.e. the function naem with the paranthesised list of parameter types (parameter types are split up by a single comma -- no spaces are used)

Starting from the fifth byte, the encoded arguments follow. This encoding is also used in other places (ie the return values and also event arguments are encoded similarly, without the four bytes specifying the function).

> more details can be found at the [relevant documentation](https://solidity.readthedocs.io/en/develop/abi-spec.html#)

## ERC 20: Token Standard Interface <a name="erc20"></a>
> [ERC 20 Token Standard Interface](https://github.com/ethereum/EIPs/blob/master/EIPS/eip-20.md)
```
contract ERC20Interface {
    function totalSupply() public constant returns (uint);
    function balanceOf(address tokenOwner) public constant returns (uint balance);
    function allowance(address tokenOwner, address spender) public constant returns (uint remaining);
    function transfer(address to, uint tokens) public returns (bool success);
    function approve(address spender, uint tokens) public returns (bool success);
    function transferFrom(address from, address to, uint tokens) public returns (bool success);

    event Transfer(address indexed from, address indexed to, uint tokens);
    event Approval(address indexed tokenOwner, address indexed spender, uint tokens);
}
```
**Implementations**<br>
* [OpenZeppelin](https://github.com/OpenZeppelin/openzeppelin-solidity/blob/9b3710465583284b8c4c5d2245749246bb2e0094/contracts/token/ERC20/ERC20.sol)
* [Consensys](https://github.com/ConsenSys/Tokens/blob/fdf687c69d998266a95f15216b1955a4965a0a6d/contracts/eip20/EIP20.sol)

## ERC 165: Standard for Detecting Interfaces <a name="erc165"></a>
> [EIP-165.md](https://github.com/ethereum/EIPs/blob/master/EIPS/eip-165.md)
This standardizes the following:
1. How interfaces are identified
2. How a contract will publish the interface it implements
3. How to detect if a contract implements ERC-165
4. How to detect if a contract implements any given interface

EIP-165 defines the interface identifier as the XOR of all function selectors in the interface. 

**Examples**<br>
* [ENS EIP-137 Standard](https://github.com/ethereum/EIPs/blob/master/EIPS/eip-137.md)

## ERC 721: Non-Fungible Token Standard <a name="erc721"></a>

## ERC 998: Composable Non-Fungible Token Standard <a name="erc998"></a>