const { ApiPromise, WsProvider, createType } = require('@polkadot/api');
const { Keyring } = require('@polkadot/keyring');

const { cryptoWaitReady } = require('@polkadot/util-crypto');

const { url, AliceAddress } = require('./config');
const { jsType } = require('./type.js');

const provider = new WsProvider(url);
const keyring = new Keyring({ type: 'sr25519' });

let api;

let AliceGoodsOperatorPublicConsumerDid = '';
let AliceGoodsOperatorPrivateConsumcerDid = '';

let Ut = require("./common");


const register = async (role) => {
	console.log('register Alice:', role);
	const alice = keyring.addFromUri('//Alice', { name: 'Alice default' });
	let category = 'ElectricMeter';
	let hash = await api.tx.operator.registerOperator(role, category).signAndSend(alice);
	console.log('registerHex:', hash.toHex());
};
const operator = async () => {
	register('PublicConsumer');
	await Ut.sleep(6000);
	register('PrivateConsumer');

	await api.query.operator.allPayOperators((value) => {
		let pos = value.toJSON();
		console.log('PayOperators:', pos);
		if (pos.length === 0) {
			// register('Payer');
		}
	});

	await api.query.operator.ownedOperators(AliceAddress, (value) => {
		let dids = value.toJSON();
		console.log('AliceDids:', dids);
		if (dids.length > 0) {
			AliceGoodsOperatorPublicConsumerDid = dids[0];
			AliceGoodsOperatorPrivateConsumcerDid = dids[1];
			api.query.operator.operators(dids[0], (obj) => {
				console.log('AliceGoodsOperatorPublicConsumer:', obj.toJSON());
			});
			api.query.operator.operators(dids[1], (obj) => {
				console.log('AliceGoodsOperatorPrivateConsumcer:', obj.toJSON());
			});
		}
	});
};

const feed_goods_oracle = async (did, feedData) => {
	const alice = keyring.addFromUri('//Alice', { name: 'Alice default' });
	let hash = await api.tx.goodsOracle.feedGoodsData(did, feedData).signAndSend(alice);
	console.log('step1.feed_goods_oracle  feed_goods hash:', hash.toHex());
};

const collector = async () => {
	const alice = keyring.addFromUri('//Alice', { name: 'Alice default' });
	let hash = await api.tx.collector.collectOracleData().signAndSend(alice);
	console.log('send collector hash:', hash.toHex());
};

const init = async () => {
	await cryptoWaitReady();

	api = await ApiPromise.create({
		provider,
		types: jsType,
	});

	await operator();
	let i = 1
	while(i<1000){
		await Ut.sleep(12000);
		console.log('*******************************************');
		console.log('simulate electric meter data:', {i}, 'times');
		console.log('*******************************************');

		
		console.log('step1.feed_goods_oracle');
		let feed1 = [ AliceGoodsOperatorPublicConsumerDid, [[ 'Peak', i*10-2 ],  [ 'Flat', i*10+3 ], [ 'valley', i*20 ] ] ];
		let feed2 = [ AliceGoodsOperatorPrivateConsumcerDid, [[ 'Peak', i*5-1 ],  [ 'Flat', i*5+2 ], [ 'valley', i*9 ] ] ];
		let feedData1 = [feed1, feed2];
		feed_goods_oracle(AliceGoodsOperatorPublicConsumerDid, feedData1);
		await Ut.sleep(6000);

		console.log('step1.feed_goods_oracle');
		let feed3 = [ AliceGoodsOperatorPublicConsumerDid, [[ 'Peak', i*20-2 ],  [ 'Flat', i*20+3 ], [ 'valley', i*40 ] ] ];
		let feed4 = [ AliceGoodsOperatorPrivateConsumcerDid, [[ 'Peak', i*8-2 ],  [ 'Flat', i*8+3 ], [ 'valley', i*18 ] ] ];
		let feedData2 = [feed3, feed4];
		feed_goods_oracle(AliceGoodsOperatorPublicConsumerDid, feedData2);
		await Ut.sleep(6000);
	
		console.log('step2.collector data');
		collector();
	
	
		let value = await api.query.collector.collectorDatas();
		console.log('step2.collector data get collector:', value.toJSON());
		   
		i++;
	}
	
};

init().catch(console.error).finally(async () => {
	console.log('init finally');
});
