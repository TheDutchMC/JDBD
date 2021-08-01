package dev.array21.jdbd.datatypes;

import java.util.HashMap;

import dev.array21.jdbd.Pair;
import dev.array21.jdbd.annotations.Nullable;

public class SqlRow {
	private final HashMap<String, Pair<Object, Class<?>>> columns = new HashMap<>();
	
	protected SqlRow(String[] names, Object[] objs, Class<?>[] classes) {
		if(names.length != objs.length || objs.length != classes.length) {
			throw new IllegalArgumentException("Length of provided arrays are not equal");
		}
		
		for(int i = 0; i < names.length; i++) {
			this.columns.put(names[i], new Pair<>(objs[i], classes[i]));
		}
	}
	
	@Nullable
	public String getString(String column) {
		return (String) getAndValidate(column, String.class);
	}
	
	@Nullable
	public Integer getInt(String column) {
		return (Integer) getAndValidate(column, Integer.class);
	}
	
	@Nullable
	public Double getDouble(String column) {
		return (Double) getAndValidate(column, Double.class);
	}
	
	@Nullable
	public Boolean getBoolean(String column) {
		return (Boolean) getAndValidate(column, Boolean.class);
	}
	
	@Nullable
	public Byte[] getBytes(String column) {
		return (Byte[]) getAndValidate(column, Byte[].class);
	}
	
	@Nullable
	private Object getAndValidate(String column, Class<?> clazz) {
		Pair<Object, Class<?>> pair = columns.get(column);
		if(pair == null) {
			return null;
		}
		
		if(pair.getB() != clazz.getClass()) {
			throw new IllegalStateException(String.format("Column '%s' was requested as String, but is %s", column, pair.getB().toString()));
		}
		
		return pair.getA();
	}
}
