#include <memory>
#include <node.h>
extern "C" {
#include <caseconv.h>
}

using namespace v8;
//using v8::FunctionCallbackInfo;
//using v8::Local;
//using v8::Object;
//using v8::String;
//using v8::Value;
//using v8::Isolate;

namespace caseconv {
	const char *to_c_string(const String::Utf8Value& value) {
		return *value ? *value : "<string conversion failed>";
	}

	void unjumble(const FunctionCallbackInfo<Value>& args) {
		Isolate* isolate = args.GetIsolate();
		if (!args[0]->IsString() || !args[1]->IsNumber()) {
			isolate->ThrowException(Exception::TypeError(String::NewFromUtf8(isolate, "expected string and number for arguments")));
			return;
		}
		String::Utf8Value src(args[0]);
		const char *c_src = to_c_string(src);
		std::unique_ptr<char> unjumbled(caseconv_unjumble(c_src, (caseconv_case_type_t)args[1]->IntegerValue()));
		Local<String> ret = String::NewFromUtf8(isolate, unjumbled.get());
		args.GetReturnValue().Set(ret);
	}

	void init(Local<Object> exports) {
		NODE_SET_METHOD(exports, "unjumble", unjumble);
	}

	NODE_MODULE(caseconv, init)
}
