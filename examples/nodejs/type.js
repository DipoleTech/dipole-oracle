const jsType = {
	Address: 'AccountId',
	LookupSource: 'AccountId',
	Did: '([u8; 32])',
	PayId: 'u32',
	OperatorRole: {
		_enum: [ 'Producer', 'Consumer', 'Payer' ],
	},
	OperatorCategory: {
		_enum: [ 'ElectricMeter', 'ChargingPoint' ],
	},
	Operator: {
		owner: 'AccountId',
		role: 'OperatorRole',
		category: 'OperatorCategory',
		is_legal: 'bool',
	},
	TimestampedVolume: {
		volume: 'u64',
		timestamp: 'Moment',
	},
	OperatorVolume: {
		operator_id: 'Did',
		volume: 'u64',
	},
	GoodsOracle: {
		oracle_operator_id: 'Did',
		init_volume: 'TimestampedVolume',
		current_volume: 'TimestampedVolume',
	},
	GoodsOracleData: {
		owner: 'AccountId',
		consumer_volume: 'u64',
		producer_volume: 'u64',
	},
	TimestampedPrice: {
		price: 'Balance',
		timestamp: 'Moment',
	},
	OperatorPrice: {
		pay_id: 'PayId',
		price: 'Balance',
	},
	PayOracle: {
		pay_id: 'PayId',
		pay_price: 'TimestampedPrice',
	},
	PayOracleData: {
		pay_id: 'PayId',
		balance: 'Balance',
	},
	CollectorData: {
		goods_oracle_data: 'Vec<GoodsOracleData>',
		pay_oracle_data: 'Vec<PayOracleData>',
	},
};
module.exports = {
	jsType,
};
