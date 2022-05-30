create table subscriptions (
	id bigserial primary key,
	user_id bigserial not null, -- user_id of the subscriber
	subscription_id text null,  -- subscription id from payment provider (e.g. stripe)
	paid_until timestamp null,  -- the timestamp the subscription is considered paid until (e.g. 1 month after payment was made)

	created_at timestamp not null default current_timestamp
);
