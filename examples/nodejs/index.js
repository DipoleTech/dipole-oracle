const { ApiPromise, WsProvider, createType } = require('@polkadot/api');
const { Keyring } = require('@polkadot/keyring');

const { cryptoWaitReady } = require('@polkadot/util-crypto');

const { url, AliceAddress } = require('./config');
const { jsType } = require('./type.js');

const provider = new WsProvider(url);
const keyring = new Keyring({ type: 'sr25519' });

let api;

let AliceGoodsOperatorDid = '';

const register = async (role) => {
	console.log('register Alice:', role);
	const alice = keyring.addFromUri('//Alice', { name: 'Alice default' });
	let category = 'ElectricMeter';
	let hash = await api.tx.operator.registerOperator(role, category).signAndSend(alice);
	console.log('registerHex:', hash.toHex());
};
const operator = async () => {
	await api.query.operator.allGoodsOperators((value) => {
		let gos = value.toJSON();
		console.log('GoodsOperators:', gos);
		if (gos.length === 0) {
			register('Producer');
		}
	});

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
			AliceGoodsOperatorDid = dids[0];
			api.query.operator.operators(dids[0], (obj) => {
				console.log('Alice:', obj.toJSON());
			});
		}
	});
};

const feed_goods_oracle = async (did, volume) => {
	let feed_operator_id = did;
	AliceGoodsOperatorDid = feed_operator_id;
	console.log('did====>', feed_operator_id);
	let feedone = [ feed_operator_id, volume ];
	const alice = keyring.addFromUri('//Alice', { name: 'Alice default' });
	let hash = await api.tx.goodsOracle.feedGoodsData(feed_operator_id, [ feedone ]).signAndSend(alice);
	console.log('feed_goods:', hash.toHex());
};

const collector = async () => {
	const alice = keyring.addFromUri('//Alice', { name: 'Alice default' });
	let hash = await api.tx.collector.collectOracleData().signAndSend(alice);
	console.log('send collector:', hash.toHex());
};

const init = async () => {
	await cryptoWaitReady();

	api = await ApiPromise.create({
		provider,
		types: jsType,
	});

	await operator();

	setTimeout(() => {
		feed_goods_oracle(AliceGoodsOperatorDid, 1000);
	}, 10000);
	setTimeout(() => {
		feed_goods_oracle(AliceGoodsOperatorDid, 1010);
	}, 20000);

	setTimeout(() => {
		collector();
	}, 30000);

	setTimeout(async () => {
		let gD = await api.query.goodsOracle.goodsDataRawValues(AliceGoodsOperatorDid);
		console.log('feed volume:', gD.toJSON());

		let value = await api.query.collector.collectorDatas();
		console.log('get collector:', value.toJSON());

		process.exit();
	}, 40000);
};

init().catch(console.error).finally(async () => {
	console.log('init finally');
});
