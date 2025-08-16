<h1 align="center">
    <img width="400px" style="border: none; margin: 0;" src="assets/capybara_2.png"/>
</h1>

### 🪙 A Bitcoin address scanner that generates keys and checks balances using a Bitcoin Core node.

<p>🔎 BTCBalanceScan is a Bitcoin wallet analysis tool that automatically generates random private keys, derives their corresponding public keys, converts them into valid Bitcoin addresses, and checks their balances directly on the blockchain.</p>

---

## **✅ Passo 1: Baixar Instalae e configurar o BitCoin Core**
1. Instale o Bitcoincore.
2. Defina para Prune da memoria para 2 blocos
3. Sincronize com a rede
```shell
#Você pode baixar uma parte da blockchain prunada
#Isso Poupa muito tempo na hora de sincronizar com a rede
C:\Users\anonymous\AppData\Roaming\Bitcoin\blocks
```
4. Ative o servidor RPC nas opções do BitCoin Core


## **✅ Passo 2: Como habilitar carteiras legacy (BDB)**

1. **Feche o Bitcoin Core** se estiver rodando.
2. **Reinicie com o seguinte parâmetro**:

```shell
bitcoind -deprecatedrpc=create_bdb
```

<br>
<br>

<h1 align="center" style="border: none; margin: 0;">Comanos BITCOIN CORE 🪙</h1>

## 💼 Para Criar nova carteira sem senha (legacy)
Execute este comando no CMD:
```shell
bitcoin-cli createwallet "wallet_0001" false false "" false false false
```

Isso cria uma carteira:

- Sem descriptors
- Sem criptografia
- Pronta para receber `importprivkey`
<br>
<br>

## 🪙 Verificar carteiras disponíveis

```shell
bitcoin-cli listwallets
```
Se "wallet_0001" não aparecer, ela não está carregada.

## 🔐 Importar chave privada 
```shell
bitcoin-cli -rpcwallet="wallet_0001" importprivkey "L21AkScUt7anosjfeg6i2oqajvLmAftDVf6jwNjiJ6GpAcynMNEG" "chave_teste" false
```
Se tudo correr bem, você verá uma resposta sem erro e sua chave estará disponível na carteira "wallet_0001".

## 🔎 Verificar o endereço associados
```shell
bitcoin-cli -rpcwallet="wallet_0001" getaddressesbylabel "chave_teste"
```

### 🎯 Você acabou de importar uma chave privada que gerou **três tipos de endereço**:

| Tipo de Endereço | Começa com | Exemplo |
| --- | --- | --- |
| Legacy (P2PKH) | `1` | `1EbBDKiAAqpqLfPoabrMU8hD5w9zNcgKcu` |
| P2SH-SegWit (P2SH) | `3` | `3BiXF6D6btG2yPvyvY3Pz9amorCBbVwssF` |
| Native SegWit (bech32) | `bc1` | `bc1qj5gympdfuak3qclffkchjanxw4um445mjhwqed` |

<br>

## **💰 Quer verificar se algum deles tem saldo?**

```shell
bitcoin-cli -rpcwallet="wallet_0001" getbalance
```
Se deu tudo certo ele vai te retornar seu saldo:  0.00000000

---
<br>

## 🧭 O que mais você pode fazer com seu nó?


1. Consultar saldo com seu nó:
```shell
bitcoin-cli getreceivedbyaddress "endereco"
```
ou
```shell
bitcoin-cli getaddressinfo "endereco"
```

2. Verificar se o endereço está na blockchain
```shell
bitcoin-cli getrawtransaction "txid" true
```

---

<br>

<h1 align="center" style="border: none; margin: 0;"> 🧭 Próximos passos que você pode seguir</h1>


### 📤 Enviar bitcoins (quando tiver saldo) 
Depois de receber fundos, você pode usar:
```shell
bitcoin-cli -rpcwallet="wallet_0001" sendtoaddress "endereco_destino" valor
```

Exemplo:
```shell
bitcoin-cli -rpcwallet="wallet_0001" sendtoaddress "bc1qxyz..." 0.001
```

## 🔐 Fazer backup da carteira
Muito importante! Para salvar sua carteira com a chave importada:
```shell
bitcoin-cli -rpcwallet="wallet_0001" backupwallet "C:\\Users\\anonymous\\backup_wallet_0001.dat"
```

## 🧮 Calcular taxa de transação ideal
```shell
bitcoin-cli estimatesmartfee 6
```
Sugere uma taxa para confirmação em até 6 blocos (~1 hora).


<h1 align="center" style="border: none; margin: 0;"> 🔐 Segurança e boas práticas</h1>

#### Use passphrase: proteja sua carteira com senha.
```shell
bitcoin-cli -rpcwallet="wallet_0001" encryptwallet "sua_senha_forte"
```

#### Verifique integridade da carteira:
```shell
bitcoin-cli -rpcwallet="wallet_0001" getwalletinfo
```